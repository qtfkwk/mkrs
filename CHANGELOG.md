# Changelog

* 0.1.0 (2023-11-04): Initial release
* 0.1.1 (2023-11-04): Fix readme
* 0.1.2 (2023-11-04): Add examples to readme
* 0.2.0 (2023-11-05): Colorized Markdown output; added `-B`, `-C`, `-f`, `-r`
  options; error on invalid target(s); update dependencies
* 0.2.1 (2023-11-06): Resolved issue [#1]; update dependencies
* 0.3.0 (2023-11-06): Added `-g` option and
  [default `Makefile.md` for a Rust project]; fixed changelog; improved readme
* 0.3.1 (2023-11-07): Improved readme and changelog
* 0.3.2 (2023-11-08): Fix error when a target file does not exist; update
  dependencies
* 0.3.3 (2023-11-08): Ignore commented commands
* 0.4.0 (2023-11-10): Add `-v` option and don't print up to date targets; move
  bunt calls to functions; improve comments and miscellaneous improvements;
  don't process dependencies for a file target unless needed (forced via `-B`,
  doesn't exist, or outdated); change default outdated response to false to
  avoid processing a file target unnecessarily

[#1]: https://github.com/qtfkwk/mkrs/issues/1
[default `Makefile.md` for a Rust project]: styles/Makefile.rust.md

