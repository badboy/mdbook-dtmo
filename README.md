# mdbook-dtmo

A combination of preprocessors for [mdbook][] to render the [Firefox Data Documentation][dtmo].

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[dtmo]: https://docs.telemetry.mozilla.org/
[gitbook]: https://github.com/GitbookIO/gitbook

## Included preprocessors:

* [mdbook-mermaid][]: add [mermaid.js][] support
* [mdbook-toc][]: add inline ToC support
* [mdbook-open-on-gh][]: add an open-on-github link on every page

[mdbook-mermaid]: https://github.com/badboy/mdbook-mermaid
[mdbook-toc]: https://github.com/badboy/mdbook-toc
[mdbook-open-on-gh]: https://github.com/badboy/mdbook-open-on-gh
[mermaid.js]: https://mermaidjs.github.io/

## Installation

If you want to use only this preprocessor, install the tool:

```
cargo install --git https://github.com/badboy/mdbook-dtmo
```

Build the book:

```
mdbook-dtmo path/to/firefox-data-docs
```

## License

MPL. See [LICENSE](LICENSE).  
Copyright (c) 2018-2020 Jan-Erik Rediger <janerik@fnordig.de>
