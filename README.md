# About

Build automation tool

* Inspired by [make]
* No automatic targets[^one]
* Minimalist functionality; maximalist readability
* Configuration is a simple Markdown file named [`Makefile.md`]
* Output is colorized Markdown (unless redirected or piped)
* Processes the target(s) specified or if none, processes the first target
* Commands run independently, in script mode, or via a custom command
* If any command fails (exits with a non-zero code), processing halts
  immediately (if not using a custom shell that does not provide this
  functionality)
* Verbosity levels: `-v`: add `-x` to `bash` command in script mode, `-vv`:
  print up to date targets, `-vvv`: show configuration
* Generates a [default `Makefile.md` for a Rust project] (`-g`)
* Lists targets via `-l`; if target(s) is specified, list hierarchical
  dependencies
* Processes targets and dependencies in the order specified

[make]: https://en.wikipedia.org/wiki/Make_(software)
[`Makefile.md`]: Makefile.md

# Syntax

## Input

* A level 1 heading begins the definition of a **target**.
* A plain text target name is a "phony" target and *always runs*.[^two]
* A code span target name is a file target and will only run if (a) any
  dependency file target's modification time is newer than the file target's,
  (b) the file target does not exist and has a recipe, or (c) force processing
  (`-B`) is enabled.[^two]
* An unordered list item defines a target **dependency**.
* A plain text dependency name is a phony dependency and will run if the target
  runs.
* A code span dependency name is a file dependency, which either has an
  associated target or not.
  If not, it is interpreted as a file glob matching existing files, which
  enables a target to easily depend on any files matching the glob, for
  instance, the `build` target may depend on `src/**/*.rs`, meaning any `*.rs`
  file under `src/`.
* A code block is a **recipe** and contains the commands that are run when the
  target is processed.
* Recipe commands run independently via `sh -c` by default,
  via `bash -eo pipefail` if script mode (`-s`) is enabled,
  via `bash -xeo pipefail` if script mode and verbose level 1 or greater (`-sv`)
  are enabled,
  or by the command given in the code block info string
* Commands may use the following variables:
    * `{0}`: first dependency
    * `{target}`: target name
    * `{dirname}`: directory name

*See [`Makefile.md`], [`styles/Makefile.rust.md`] and/or the `-g` option for
examples.*

[`styles/Makefile.rust.md`]: styles/Makefile.rust.md

## Output

* A level 2 heading is the output section: "Configuration", "Target(s)".
* A Level 3 heading in the Target(s) section is each target, either as plain
  text "phony" target or a code span file target.
* Code blocks:

    Script Mode | Dry Run | Description
    ------------|---------|------------------------------------------------
    &nbsp;      |         | Each command and output
    &nbsp;      | ✔       | Each command
    ✔           |         | Each script
    ✔           | ✔       | Each script and output (in separate code block)

# Usage

~~~text
$ mkrs -V
mkrs 0.16.1
~~~

~~~text
$ mkrs -h
Build automation tool

Usage: mkrs [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Target(s)

Options:
  -l              List targets/dependencies
  -B              Force processing
  -n              Dry run
  -s              Script mode
  -v...           Verbose
  -q              Quiet
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
* all
* check
* update
* run
* clippy
* test
* build
* `README.md`
* doc
* outdated
* audit
* update-toml
* update-lock
* install
* uninstall
* install-deps
* scaffold
* clean
* cocomo
* full
* fail
* `nonexistent`
* custom

~~~

## List dependencies for `full` target

~~~text
$ mkrs -l full
* full
    * update
        * update-toml
        * update-lock
    * check
        * outdated
        * audit
    * all
        * clippy
            * `Cargo.lock`
            * `Cargo.toml`
            * `src/main.rs`
        * test
            * `Cargo.lock`
            * `Cargo.toml`
            * `src/main.rs`
        * build
            * `Cargo.lock`
            * `Cargo.toml`
            * `src/main.rs`
            * `README.md`
                * `t/README.md`
                * `Cargo.toml`
                * `CHANGELOG.md`
                * `src/main.rs`
        * doc
    * install
        * `README.md`
            * `t/README.md`
            * `Cargo.toml`
            * `CHANGELOG.md`
            * `src/main.rs`

~~~

## Dry run

~~~text
$ mkrs -n
# clippy

```text
cargo clippy -- -D clippy::all
```

# test

```text
cargo test --release
```

# build

```text
cargo build --release
```

# doc

```text
cargo doc
```

~~~

## Process default target

~~~text
$ mkrs
# clippy

```text
$ cargo clippy -- -D clippy::all
    Checking mkrs v0.16.1 (/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.36s
```

# test

```text
$ cargo test --release
   Compiling mkrs v0.16.1 (/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 0.44s
     Running unittests src/main.rs (target/release/deps/mkrs-0c5fe6eb09ce86ff)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

# build

```text
$ cargo build --release
   Compiling mkrs v0.16.1 (/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.39s
```

# doc

```text
$ cargo doc
 Documenting mkrs v0.16.1 (/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.38s
   Generated /home/nick/github.com/qtfkwk/mkrs/target/doc/mkrs/index.html
```

~~~

## Process `check` target

~~~text
$ mkrs check
# outdated

```text
$ cargo outdated --exit-code=1
All dependencies are up to date, yay!
```

# audit

```text
$ cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 629 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (80 crate dependencies)
```

~~~

## Process `update`, `check`, and `build` targets

~~~text
$ mkrs update check build
# update-toml

```text
$ cargo upgrade -i
    Updating 'https://github.com/rust-lang/crates.io-index' index
    Checking mkrs's dependencies
note: Re-run with `--verbose` to show more dependencies
  latest: 10 packages
```

# update-lock

```text
$ cargo update
    Updating crates.io index
     Locking 0 packages to latest compatible versions
note: pass `--verbose` to see 13 unchanged dependencies behind latest
```

# outdated

```text
$ cargo outdated --exit-code=1
All dependencies are up to date, yay!
```

# audit

```text
$ cargo audit
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 629 security advisories (from /home/nick/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (80 crate dependencies)
```

# build

```text
$ cargo build --release
   Compiling mkrs v0.16.1 (/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.40s
```

~~~

## Generate a default Makefile.md for a Rust project

~~~text
$ mkrs -g rust
# all

* clippy
* test
* build
* doc

# check

* outdated
* audit

# update

* update-toml
* update-lock

# run

* build
* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`

```
cargo test --release
```

# build

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `src/**/*.rs`

```
cargo build --release
kapow {0} >{target}
```

# doc

```
cargo doc
```

# outdated

```
cargo outdated --exit-code=1
```

# audit

```
cargo audit
```

# update-toml

```
cargo upgrade -i
```

# update-lock

```
cargo update
```

# install

* `README.md`

```
cargo install --path .
```

# uninstall

```
cargo uninstall {dirname}
```

# install-deps

```
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow tokei toml-cli
```

# scaffold

```bash -eo pipefail
if ! toml get -r Cargo.toml package.description >/dev/null; then
toml set Cargo.toml package.description "Insert a description here" >Cargo.toml.new
mv Cargo.toml.new Cargo.toml
echo Edit package description in Cargo.toml, then rerun \`mkrs scaffold\`.
exit 0
fi
mkdir -p t
if [ ! -e t/README.md ]; then
NAME=$(toml get -r Cargo.toml package.name)
ABOUT=$(toml get -r Cargo.toml package.description)
cat <<EOF >t/README.md
# About

$ABOUT

# Usage

~~~~text
\$ $NAME -V
!run:../target/release/$NAME -V 2>&1
~~~~

~~~~text
\$ $NAME -h
!run:../target/release/$NAME -h 2>&1
~~~~

!inc:../CHANGELOG.md

EOF
fi
if [ ! -e CHANGELOG.md ]; then
VERSION=$(toml get -r Cargo.toml package.version)
TODAY=$(dtg -n %Y-%m-%d)
cat <<EOF >CHANGELOG.md
# Changelog

* $VERSION ($TODAY): Initial release

EOF
fi
```

# clean

```
cargo clean
```

# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

# full

* update
* check
* all
* install

~~~

**Note:** Save to `Makefile.md` via redirection: `mkrs -g rust >Makefile.md`

## Generate a COCOMO report

~~~text
$ mkrs cocomo
# cocomo

```bash -eo pipefail
tokei; echo
cocomo -o sloccount
cocomo
```

```text
===============================================================================
 Language            Files        Lines         Code     Comments       Blanks
===============================================================================
 TOML                    1           21           19            0            2
-------------------------------------------------------------------------------
 Markdown                5         1007            0          746          261
 |- BASH                 3          142          109            9           24
 |- Python               1            1            1            0            0
 (Total)                           1150          110          755          285
-------------------------------------------------------------------------------
 Rust                    1          654          550           34           70
 |- Markdown             1           11            0           11            0
 (Total)                            665          550           45           70
===============================================================================
 Total                   7         1682          569          780          333
===============================================================================

Total Physical Source Lines of Code (SLOC)                    = 569
Development Effort Estimate, Person-Years (Person-Months)     = 0.11 (1.33)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.23 (2.78)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 0.48
Total Estimated Cost to Develop                               = $14,945
  (average salary = $56,286/year, overhead = 2.40)

Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 569
Estimated Cost to Develop  | $14,945.47
Estimated Schedule Effort  | 2.78 months
Estimated People Required  | 0.48

```

~~~

## Use a custom shell program

~~~text
$ mkrs custom
# custom

```python
print("This is a custom recipe in Python.")
```

```text
This is a custom recipe in Python.
```

~~~

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
* 0.14.0 (2024-04-21): Remove useless level 1-2 headings; update dependencies
* 0.15.0 (2024-06-19): Add -q option; update dependencies
* 0.16.0 (2024-06-22): Add `dirname` variable; update dependencies
    * 0.16.1 (2024-06-22): Make `uninstall` target use `dirname` variable;
      correct dependency ordering for `README.md` target

[default `Makefile.md` for a Rust project]: styles/Makefile.rust.md
[#1]: https://github.com/qtfkwk/mkrs/issues/1
[`glob`]: https://crates.io/crates/glob

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to
*compile* any sort of file; all such commands must be defined in the
configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the
target(s) that are being processed.

