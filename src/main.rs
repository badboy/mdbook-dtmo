extern crate chrono;
#[macro_use]
extern crate clap;
extern crate env_logger;
#[macro_use]
extern crate log;
extern crate mdbook;
extern crate mdbook_mermaid;
extern crate mdbook_toc;
extern crate open;

use chrono::Local;
use clap::{App, AppSettings, ArgMatches};
use env_logger::Builder;
use log::LevelFilter;
use mdbook::utils;
use std::env;
use std::ffi::OsStr;
use std::io::Write;
use std::path::{Path, PathBuf};

mod cmd;

const NAME: &'static str = "mdbook-dtmo";
const VERSION: &'static str = concat!("v", crate_version!());

fn main() {
    init_logger();
    //
    // Create a list of valid arguments and sub-commands
    let app = App::new(NAME)
        .about("Creates a book from markdown files with added plugins")
        .author("Jan-Erik Rediger <jrediger@mozilla.com>")
        .version(VERSION)
        .setting(AppSettings::GlobalVersion)
        .setting(AppSettings::ArgRequiredElseHelp)
        .after_help(
            "For more information about a specific command, try `mdbook-dtmo <command> --help`\n\
             The source code for mdBook is available at: https://github.com/badboy/mdbook-dtmo",
        )
        .subcommand(cmd::init::make_subcommand())
        .subcommand(cmd::build::make_subcommand())
        .subcommand(cmd::clean::make_subcommand())
        .subcommand(cmd::watch::make_subcommand())
        .subcommand(cmd::serve::make_subcommand());

    // Check which subcomamnd the user ran...
    let res = match app.get_matches().subcommand() {
        ("init", Some(sub_matches)) => cmd::init::execute(sub_matches),
        ("build", Some(sub_matches)) => cmd::build::execute(sub_matches),
        ("clean", Some(sub_matches)) => cmd::clean::execute(sub_matches),
        ("watch", Some(sub_matches)) => cmd::watch::execute(sub_matches),
        ("serve", Some(sub_matches)) => cmd::serve::execute(sub_matches),
        (_, _) => unreachable!(),
    };

    if let Err(e) = res {
        utils::log_backtrace(&e);

        ::std::process::exit(101);
    }
}

fn init_logger() {
    let mut builder = Builder::new();

    builder.format(|formatter, record| {
        writeln!(
            formatter,
            "{} [{}] ({}): {}",
            Local::now().format("%Y-%m-%d %H:%M:%S"),
            record.level(),
            record.target(),
            record.args()
        )
    });

    if let Ok(var) = env::var("RUST_LOG") {
        builder.parse(&var);
    } else {
        // if no RUST_LOG provided, default to logging at the Info level
        builder.filter(None, LevelFilter::Info);
        // Filter extraneous html5ever not-implemented messages
        builder.filter(Some("html5ever"), LevelFilter::Error);
    }

    builder.init();
}

fn get_book_dir(args: &ArgMatches) -> PathBuf {
    if let Some(dir) = args.value_of("dir") {
        // Check if path is relative from current dir, or absolute...
        let p = Path::new(dir);
        if p.is_relative() {
            env::current_dir().unwrap().join(dir)
        } else {
            p.to_path_buf()
        }
    } else {
        env::current_dir().expect("Unable to determine the current directory")
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    if let Err(e) = open::that(path) {
        error!("Error opening web browser: {}", e);
    }
}

pub fn make_app<'a, 'b>() -> App<'a, 'b> {
    App::new("mdbook-dtmo")
        .about("Build the book from the markdown files for dtmo")
        .arg_from_usage(
            "-d, --dest-dir=[dest-dir] 'The output directory for your book{n}(Defaults to ./book \
             when omitted)'",
        )
        .arg_from_usage(
            "[dir] 'A directory for your book{n}(Defaults to Current Directory when omitted)'",
        )
}
