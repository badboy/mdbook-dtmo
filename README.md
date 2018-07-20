# mdbook-dtmo

A combination of preprocessors for [mdbook][] to render the [Firefox Data Documentation][dtmo-experiment] (experimental*).

_*) Note: The [official documentation][dtmo] currently uses [gitbook][]. The switch to mdbook is an experiment looking for alternatives, as gitbook and its CLI are deprecated. The mdbook variant is available in [the forked repository](https://github.com/badboy/firefox-data-docs/compare/master...badboy:mdbook)._

[mdbook]: https://github.com/rust-lang-nursery/mdBook
[dtmo-experiment]: https://badboy.github.io/firefox-data-docs/
[dtmo]: https://docs.telemetry.mozilla.org/
[gitbook]: https://github.com/GitbookIO/gitbook

## Included preprocessors:

* [mdbook-mermaid][]: add [mermaid.js][] support
* [mdbook-toc][]: add inline ToC support

[mdbook-mermaid]: https://github.com/badboy/mdbook-mermaid
[mdbook-toc]: https://github.com/badboy/mdbook-toc
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
Copyright (c) 2018 Jan-Erik Rediger <janerik@fnordig.de>
