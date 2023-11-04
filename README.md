# About

Build automation tool

* Inspired by [make]
* No automatic targets[^one]
* Minimalist functionality; maximalist readability
* Configuration is a simple Markdown file named [`Makefile.md`]
* Processes the target(s) specified or if none, processes the first target
* Commands are run via `sh` shell
* If any command fails (exits with a non-zero code), processing halts
  immediately

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

```
$ mkrs -V
mkrs 0.1.2
```

```
$ mkrs -h
Build automation tool

Usage: mkrs [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Target(s)

Options:
  -l             List available targets
  -n             Dry run
  -h, --help     Print help
  -V, --version  Print version
```

# Examples

```
$ mkrs -l
build
README.md
clippy
test
check
update
install
uninstall
install-deps
fail
```

```
$ mkrs -n
cargo build --release
kapow t/README.md >README.md
cargo clippy -- -D clippy::all
cargo build --release
```

```
$ mkrs
cargo build --release
    Finished release [optimized] target(s) in 0.02s
kapow t/README.md >README.md
cargo clippy -- -D clippy::all
    Finished dev [unoptimized + debuginfo] target(s) in 0.07s
cargo build --release
    Finished release [optimized] target(s) in 0.07s
```

```
$ mkrs check
cargo outdated --exit-code 1
All dependencies are up to date, yay!
cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 576 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (47 crate dependencies)
```

```
$ mkrs update check build
cargo upgrade --incompatible
    Updating 'https://github.com/rust-lang/crates.io-index' index
    Checking mkrs's dependencies
note: Re-run with `--verbose` to show more dependencies
  latest: 5 packages
cargo update
    Updating crates.io index
cargo outdated --exit-code 1
All dependencies are up to date, yay!
cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 576 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (47 crate dependencies)
cargo build --release
    Finished release [optimized] target(s) in 0.03s
kapow t/README.md >README.md
cargo clippy -- -D clippy::all
    Finished dev [unoptimized + debuginfo] target(s) in 0.06s
cargo build --release
    Finished release [optimized] target(s) in 0.06s
```

---

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to
*compile* any sort of file; all such commands must be defined in the
configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the
target(s) that are being processed.

