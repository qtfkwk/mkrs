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
* 0.5.0 (2023-11-10): Fail to run on Windows; ignore leading/trailing whitespace
  in commands; append commands instead of replacing them; improve readme; add
  `-s` (script mode)
* 0.6.0 (2023-11-11): Use [`glob`] crate to process file dependencies without
  targets; `-vvv`: print `Config`; fix changelog; improve readme; add `clean`
  target to Makefiles; update dependencies
* 0.7.0 (2023-11-11): Make dependency ordering significant; trace dependencies
  for specified targets for `-l`; add `full` target to Makefiles; add
  `README.md` dependency on `install` target; don't print phony targets without
  commands or `-vv`; fix readme
* 0.8.0 (2023-11-11): Add `cocomo` target to Makefiles
* 0.9.0 (2023-11-12): Enable using the code block info string to define a custom
  shell command; fix issue running multiple targets specified on command line;
  improve readme; update dependencies
* 0.10.0 (2023-11-13): Treat recipes with a custom shell command as a script
  rather than individual commands
* 0.11.0 (2023-11-20): Fix the globbing a nonexistent file dependency results in
  zero dependencies issue; update dependencies
* 0.12.0 (2023-12-04): Add `scaffold` target; update dependencies
    * 0.12.1 (2023-12-04): Fix scaffold target; update dependencies
* 0.13.0 (2024-01-05): Use sprint; update dependencies
    * 0.13.1 (2024-01-27): Fix issue where a failed command did not halt
      processing; update dependencies

[default `Makefile.md` for a Rust project]: styles/Makefile.rust.md
[#1]: https://github.com/qtfkwk/mkrs/issues/1
[`glob`]: https://crates.io/crates/glob

