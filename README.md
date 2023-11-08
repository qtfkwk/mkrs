# About

Build automation tool

* Inspired by [make]
* No automatic targets[^one]
* Minimalist functionality; maximalist readability
* Configuration is a simple Markdown file named [`Makefile.md`]
* Output is colorized Markdown
* Processes the target(s) specified or if none, processes the first target
* Commands are run via `sh` shell
* If any command fails (exits with a non-zero code), processing halts
  immediately
* Generates a [default `Makefile.md` for a Rust project]

[make]: https://en.wikipedia.org/wiki/Make_(software)
[`Makefile.md`]: Makefile.md

# Syntax

* A Level 1 heading begins the definition of a **target**.
* A plain text target name is a "phony" target and will *always run*.[^two]
* A code span target name is a file target and will only run if any dependency
  file target's modification time is newer than the target.[^two]
* An unordered list contains the target's dependencies.
* A code block contains the commands that are run when the target is processed.
* Commands may use the following variables:
    * `{0}`: first dependency
    * `{target}`: target name

*See [`Makefile.md`] for an example.*

# Usage

~~~text
$ mkrs -V
mkrs 0.3.3
~~~

~~~text
$ mkrs -h
Build automation tool

Usage: mkrs [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Target(s)

Options:
  -l              List available targets
  -B              Force processing
  -n              Dry run
  -C <PATH>       Change directory
  -f <PATH>       Configuration file [default: Makefile.md]
  -g <STYLE>      Generate Makefile.md content [styles: rust]
  -r              Print readme
  -h, --help      Print help
  -V, --version   Print version
~~~

# Examples

## List available targets

~~~text
$ mkrs -l
# Targets

* build
* `README.md`
* clippy
* test
* check
* update
* install
* uninstall
* install-deps
* fail
* `nonexistent`

~~~

## Dry run

~~~text
$ mkrs -n
# `README.md`

*Up to date*

# clippy

```text
$ cargo clippy -- -D clippy::all
```

# build

```text
$ cargo build --release
```

~~~

## Process default target

~~~text
$ mkrs
# `README.md`

*Up to date*

# clippy

```text
$ cargo clippy -- -D clippy::all
    Checking mkrs v0.3.3 (/home/nick/github.com/qtfkwk/mkrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
```

# build

```text
$ cargo build --release
   Compiling mkrs v0.3.3 (/home/nick/github.com/qtfkwk/mkrs)
    Finished release [optimized] target(s) in 1.36s
```

~~~

## Process `check` target

~~~text
$ mkrs check
# check

```text
$ cargo outdated --exit-code 1
All dependencies are up to date, yay!
```

```text
$ cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 577 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (61 crate dependencies)
```

~~~

## Process `update`, `check`, and `build` targets

~~~text
$ mkrs update check build
# update

```text
$ cargo upgrade --incompatible
    Updating 'https://github.com/rust-lang/crates.io-index' index
    Checking mkrs's dependencies
note: Re-run with `--verbose` to show more dependencies
  latest: 8 packages
```

```text
$ cargo update
    Updating crates.io index
```

# check

```text
$ cargo outdated --exit-code 1
All dependencies are up to date, yay!
```

```text
$ cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 577 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (61 crate dependencies)
```

# `README.md`

*Up to date*

# clippy

```text
$ cargo clippy -- -D clippy::all
    Checking mkrs v0.3.3 (/home/nick/github.com/qtfkwk/mkrs)
    Finished dev [unoptimized + debuginfo] target(s) in 0.26s
```

# build

```text
$ cargo build --release
   Compiling mkrs v0.3.3 (/home/nick/github.com/qtfkwk/mkrs)
    Finished release [optimized] target(s) in 1.35s
```

~~~

## Generate a default Makefile.md for a Rust project

~~~text
$ mkrs -g rust
# build

* clippy
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`

```
cargo build --release
kapow {0} >{target}
```

# clippy

```
cargo clippy -- -D clippy::all
```

# test

```
cargo test
```

# check

```
cargo outdated --exit-code 1
cargo audit
```

# update

```
cargo upgrade --incompatible
cargo update
```

# install

```
cargo install --path .
```

# uninstall

```
cargo uninstall $(toml get -r Cargo.toml package.name)
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated kapow toml-cli
```

~~~

**Note:** Save to `Makefile.md` via redirection: `mkrs -g rust >Makefile.md`

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

[#1]: https://github.com/qtfkwk/mkrs/issues/1
[default `Makefile.md` for a Rust project]: styles/Makefile.rust.md

---

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to
*compile* any sort of file; all such commands must be defined in the
configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the
target(s) that are being processed.

