extern crate iron;
extern crate staticfile;
extern crate ws;

use self::iron::{
    status, AfterMiddleware, Chain, Iron, IronError, IronResult, Request, Response, Set,
};
use super::watch;
use clap::{App, Arg, ArgMatches, SubCommand};
use mdbook::errors::*;
use mdbook::utils;
use mdbook::MDBook;
use mdbook_mermaid::Mermaid;
use mdbook_toc::Toc;
use std;
use {get_book_dir, open};

struct ErrorRecover;

// Create clap subcommand arguments
pub fn make_subcommand<'a, 'b>() -> App<'a, 'b> {
    SubCommand::with_name("serve")
        .about("Serves a book at http://localhost:3000, and rebuilds it on changes")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'Output directory for the book{n}\
             (If omitted, uses build.build-dir from book.toml or defaults to ./book)'",
        )
        .arg_from_usage(
            "[dir] 'Root directory for the book{n}\
             (Defaults to the Current Directory when omitted)'",
        )
        .arg(
            Arg::with_name("hostname")
                .short("n")
                .long("hostname")
                .takes_value(true)
                .default_value("localhost")
                .empty_values(false)
                .help("Hostname to listen on for HTTP connections"),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .takes_value(true)
                .default_value("3000")
                .empty_values(false)
                .help("Port to use for HTTP connections"),
        )
        .arg(
            Arg::with_name("websocket-hostname")
                .long("websocket-hostname")
                .takes_value(true)
                .empty_values(false)
                .help(
                    "Hostname to connect to for WebSockets connections (Defaults to the HTTP hostname)",
                ),
        )
        .arg(
            Arg::with_name("websocket-port")
                .short("w")
                .long("websocket-port")
                .takes_value(true)
                .default_value("3001")
                .empty_values(false)
                .help("Port to use for WebSockets livereload connections"),
        )
        .arg_from_usage("-o, --open 'Opens the book server in a web browser'")
}

// Watch command implementation
pub fn execute(args: &ArgMatches) -> Result<()> {
    let book_dir = get_book_dir(args);
    let mut book = MDBook::load(&book_dir)?;

    let port = args.value_of("port").unwrap();
    let ws_port = args.value_of("websocket-port").unwrap();
    let hostname = args.value_of("hostname").unwrap();
    let public_address = args.value_of("websocket-address").unwrap_or(hostname);
    let open_browser = args.is_present("open");

    let address = format!("{}:{}", hostname, port);
    let ws_address = format!("{}:{}", hostname, ws_port);

    let livereload_url = format!("ws://{}:{}", public_address, ws_port);
    book.config
        .set("output.html.livereload-url", &livereload_url)?;

    if let Some(dest_dir) = args.value_of("dest-dir") {
        book.config.build.build_dir = dest_dir.into();
    }

    book.with_preprecessor(Mermaid);
    book.with_preprecessor(Toc);
    book.build()?;

    let mut chain = Chain::new(staticfile::Static::new(book.build_dir_for("html")));
    chain.link_after(ErrorRecover);
    let _iron = Iron::new(chain)
        .http(&*address)
        .chain_err(|| "Unable to launch the server")?;

    let ws_server =
        ws::WebSocket::new(|_| |_| Ok(())).chain_err(|| "Unable to start the websocket")?;

    let broadcaster = ws_server.broadcaster();

    std::thread::spawn(move || {
        ws_server.listen(&*ws_address).unwrap();
    });

    let serving_url = format!("http://{}", address);
    info!("Serving on: {}", serving_url);

    if open_browser {
        open(serving_url);
    }

    watch::trigger_on_change(&mut book, move |path, book_dir| {
        info!("File changed: {:?}", path);
        info!("Building book...");

        // FIXME: This area is really ugly because we need to re-set livereload :(

        let result = MDBook::load(&book_dir)
            .and_then(|mut b| {
                b.config
                    .set("output.html.livereload-url", &livereload_url)?;
                b.with_preprecessor(Mermaid);
                b.with_preprecessor(Toc);
                Ok(b)
            }).and_then(|b| b.build());

        if let Err(e) = result {
            error!("Unable to load the book");
            utils::log_backtrace(&e);
        } else {
            let _ = broadcaster.send("reload");
        }
    });

    Ok(())
}

impl AfterMiddleware for ErrorRecover {
    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        match err.response.status {
            // each error will result in 404 response
            Some(_) => Ok(err.response.set(status::NotFound)),
            _ => Err(err),
        }
    }
}
