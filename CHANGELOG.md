# Changelog

* 0.1.0 (2023-11-04): Initial release
    * 0.1.1 (2023-11-04): Fix readme
    * 0.1.2 (2023-11-04): Add examples to readme
* 0.2.0 (2023-11-05): Colorized Markdown output; added `-B`, `-C`, `-f`, `-r` options; error on invalid target(s); update dependencies
    * 0.2.1 (2023-11-06): Resolved issue [#1]; update dependencies
* 0.3.0 (2023-11-06): Added `-g` option and [default `Makefile.md` for a Rust project]; fixed changelog; improved readme
    * 0.3.1 (2023-11-07): Improved readme and changelog
    * 0.3.2 (2023-11-08): Fix error when a target file does not exist; update dependencies
    * 0.3.3 (2023-11-08): Ignore commented commands
* 0.4.0 (2023-11-10): Add `-v` option and don't print up to date targets; move bunt calls to functions; improve comments and miscellaneous improvements; don't process dependencies for a file target unless needed (forced via `-B`, doesn't exist, or outdated); change default outdated response to false to avoid processing a file target unnecessarily
* 0.5.0 (2023-11-10): Fail to run on Windows; ignore leading/trailing whitespace in commands; append commands instead of replacing them; improve readme; add `-s` (script mode)
* 0.6.0 (2023-11-11): Use [`glob`] crate to process file dependencies without targets; `-vvv`: print `Config`; fix changelog; improve readme; add `clean` target to Makefiles; update dependencies
* 0.7.0 (2023-11-11): Make dependency ordering significant; trace dependencies for specified targets for `-l`; add `full` target to Makefiles; add `README.md` dependency on `install` target; don't print phony targets without commands or `-vv`; fix readme
* 0.8.0 (2023-11-11): Add `cocomo` target to Makefiles
* 0.9.0 (2023-11-12): Enable using the code block info string to define a custom shell command; fix issue running multiple targets specified on command line; improve readme; update dependencies
* 0.10.0 (2023-11-13): Treat recipes with a custom shell command as a script rather than individual commands
* 0.11.0 (2023-11-20): Fix the globbing a nonexistent file dependency results in zero dependencies issue; update dependencies
* 0.12.0 (2023-12-04): Add `scaffold` target; update dependencies
    * 0.12.1 (2023-12-04): Fix scaffold target; update dependencies
* 0.13.0 (2024-01-05): Use sprint; update dependencies
    * 0.13.1 (2024-01-27): Fix issue where a failed command did not halt processing; update dependencies
* 0.14.0 (2024-04-21): Remove useless level 1-2 headings; update dependencies
* 0.15.0 (2024-06-19): Add -q option; update dependencies
* 0.16.0 (2024-06-22): Add `dirname` variable; update dependencies
    * 0.16.1 (2024-06-22): Make `uninstall` target use `dirname` variable; correct dependency ordering for `README.md` target
    * 0.16.2 (2024-06-22): Remove `run` target; fix `run` target in generated Rust configuration
    * 0.16.3 (2024-06-23): Fix issue when using `dirname` variable in file target name; update dependencies
    * 0.16.4 (2024-06-23): Fix `build` target in generated Rust configuration
    * 0.16.5 (2024-07-03): Add `.env` / dotenv example to readme; update dependencies
* 0.17.0 (2024-08-04): Switch terminal colors from [`bunt`] to [`owo-colors`] ([ref][rain-rust-cli-colors]); add `--color` option; fix makefiles; update dependencies
    * 0.17.1 (2024-08-06): Add `cprint`, `ecprint` macros; don't include changelog in the readme
* 0.18.0 (2024-08-16): Enable multiple configuration files; use `-C` to change directories instead of changing to configuration file parent directory; fix changelog; update dependencies
    * 0.18.1 (2024-08-21): Update dependencies
    * 0.18.2 (2024-08-22): Add `commit` target to makefiles; update dependencies
    * 0.18.3 (2024-09-05): Fix readme; update dependencies
* 0.19.0 (2024-10-25): Add clap color; switch from owo-colors' support-colors feature to [`anstream`]; update dependencies
    * 0.19.1 (2024-10-26): Fix clap color
    * 0.19.2 (2024-12-04): Update dependencies
* 0.20.0 (2024-12-08): Enable tilde expansion user home directory via [`expanduser`] in file dependencies; update dependencies
    * 0.20.1 (2024-12-19): Update dependencies
* 0.21.0 (2025-01-04): Enable wildcard / glob targets; update dependencies
* 0.22.0 (2025-01-12): Print wildcard targets as code spans; enumerate dependencies built via wildcard target; resolve wildcard targets after loading markdown versus when processing targets; renumber errors; update dependencies
    * 0.22.1 (2025-01-14): Fix documentation; fix print_code_block macro; update dependencies
    * 0.22.2 (2025-02-21): Update dependencies
* 0.23.0 (2025-02-22): Add `serve-doc` target; updated dependencies
    * 0.23.1 (2025-04-16): Update dependencies
* 0.24.0 (2025-08-28): Update dependencies; 2024 edition
    * 0.24.1 (2025-10-27): Update dependencies; use [`pager2`]
    * 0.24.2 (2025-11-11): Update dependencies; use [`clap-cargo`] `CLAP_STYLING`; replace lazy_static with LazyLock; clippy pedantic fixes; cargo fmt
    * 0.24.3 (2025-11-11): Update dependencies

[default `Makefile.md` for a Rust project]: styles/Makefile.rust.md
[#1]: https://github.com/qtfkwk/mkrs/issues/1
[`anstream`]: https://crates.io/crates/anstream
[`glob`]: https://crates.io/crates/glob
[`bunt`]: https://crates.io/crates/bunt
[`owo-colors`]: https://crates.io/crates/owo-colors
[`expanduser`]: https://crates.io/crates/expanduser
[`pager2`]: https://crates.io/crates/pager2
[`clap-cargo`]: https://crates.io/crates/clap-cargo
[rain-rust-cli-colors]: https://rust-cli-recommendations.sunshowers.io/managing-colors-in-rust.html

