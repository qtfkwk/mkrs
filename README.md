# About

Build automation tool

* Inspired by [make]
* No automatic targets[^one]
* Minimalist functionality; maximalist readability
* Configuration is one or more simple Markdown files (`Makefile.md` by default)
* Output is colorized Markdown (unless redirected or piped)
* Processes the target(s) specified or if none, processes the first target
* Commands run independently, in script mode, or via a custom command
* If any command fails (exits with a non-zero code), processing halts immediately (if not using a
  custom shell that does not provide this functionality)
* Verbosity levels:
    * `-v`: add `-x` to `bash` command in script mode
    * `-vv`: print up to date targets
    * `-vvv`: show configuration
* Generates a default `Makefile.md` for a Rust project via `-g rust`
* Lists targets via `-l`; if target(s) is specified, list hierarchical dependencies
* Processes targets and dependencies in the order specified
* Designed to work flexibly with other shells, scripting languages, and utilities like [`dotenv`]

[make]: https://en.wikipedia.org/wiki/Make_(software)
[`Makefile.md`]: Makefile.md

# Syntax

## Input

* A level 1 heading begins the definition of a **target**.
* A plain text target name is a "phony" target and *always runs*.[^two]
* A code span target name is a **file target** and will only run if
  (a) any dependency file target's modification time is newer than the file target's,
  (b) the file target does not exist and has a recipe, or
  (c) force processing (`-B`) is enabled.[^two]
* An unordered list item defines a target **dependency**.
* A plain text dependency name is a phony dependency and will run if the target runs.
* A code span dependency name is a file dependency, which either has an associated target or not.
  If not, it is interpreted as a file glob matching existing files, which enables a target to easily
  depend on any files matching the glob, for instance, the `build` target may depend on `**/*.rs`,
  meaning any `*.rs` file under `./`.
* A code block is a **recipe** and contains the commands that are run when the target is processed.
* Recipe commands run independently via `sh -c` by default,
  via `bash -eo pipefail` if script mode (`-s`) is enabled,
  via `bash -xeo pipefail` if script mode and verbose level 1 or greater (`-sv`) are enabled,
  or by the command given in the code block info string.
* Commands may use the following variables:
    * `{0}`: first dependency
    * `{target}`: target name
    * `{dirname}`: directory name
* A **file target** that is a `*.ext` glob is a **wildcard target** whose **recipe** is used for any
  matching **dependency** in the `Makefile.md` or **target** on the command line that does not have
  its own **recipe**.
* If a **file target**'s first dependency is a `*.ext` glob, it is interpreted as being the same
  path as the **file target** except with the given extension.

*See [`Makefile.md`], [`styles/Makefile.rust.md`] and/or the `-g` option for examples.*

[`styles/Makefile.rust.md`]: styles/Makefile.rust.md

## Output

* A level 2 heading is the output section: "Configuration", "Target(s)".
* A Level 3 heading in the Target(s) section is each target, either as plain text "phony" target or
  a code span file target.
* Code blocks:

    Script Mode | Dry Run | Description
    ------------|---------|------------------------------------------------
    &nbsp;      | &nbsp;  | Each command and output
    &nbsp;      | ✔       | Each command
    ✔           | &nbsp;  | Each script and output (in separate code blocks)
    ✔           | ✔       | Each script

# Usage

~~~text
$ mkrs -V
mkrs 0.23.1
~~~

~~~text
$ mkrs -h
Build automation tool

Usage: mkrs [OPTIONS] [NAME]...

Arguments:
  [NAME]...  Target(s)

Options:
  -l                   List targets/dependencies
  -B                   Force processing
  -n                   Dry run
  -s                   Script mode
  -v...                Verbose
  -q                   Quiet
  -C <PATH>            Change directory
  -f <PATH>            Configuration file(s) [default: Makefile.md]
  -g <STYLE>           Generate Makefile.md content [styles: rust]
      --color <COLOR>  Force enable/disable terminal colors [default: auto]
                       [possible values: auto, always, never]
  -r                   Print readme
  -h, --help           Print help
  -V, --version        Print version
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
* `target/release/mkrs`
* `README.md`
* doc
* serve-doc
* outdated
* audit
* update-toml
* update-lock
* install
* uninstall
* install-deps
* clean
* cocomo
* commit
* publish
* full
* fail
* `nonexistent`
* custom
* `*.png`
* `img/crates.png`

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
            * `target/release/mkrs`
                * `Cargo.lock`
                * `Cargo.toml`
                * `src/main.rs`
                * `README.md`
                    * `t/README.md`
                    * `Cargo.toml`
                    * `CHANGELOG.md`
                    * `src/main.rs`
                    * `img/crates.png`
                        * `img/crates.gv`
        * doc
    * install
        * `README.md`
            * `t/README.md`
            * `Cargo.toml`
            * `CHANGELOG.md`
            * `src/main.rs`
            * `img/crates.png`
                * `img/crates.gv`

~~~

## Dry run

~~~text
$ mkrs -n
# `target/release/mkrs`

```text
cargo build --release
```

# clippy

```text
cargo clippy -- -D clippy::all
```

# test

```text
cargo test
```

# doc

```text
cargo doc
```

~~~

## Process default target

~~~text
$ mkrs
# `target/release/mkrs`

```text
$ cargo build --release
   Compiling mkrs v0.23.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.88s
```

# clippy

```text
$ cargo clippy -- -D clippy::all
    Checking mkrs v0.23.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
```

# test

```text
$ cargo test
   Compiling mkrs v0.23.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.35s
     Running unittests src/main.rs (target/debug/deps/mkrs-176f35821763f9e2)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

# doc

```text
$ cargo doc
 Documenting mkrs v0.23.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.41s
   Generated /media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs/target/doc/mkrs/index.html
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
[0m[0m[1m[32m    Fetching[0m advisory database from `https://github.com/RustSec/advisory-db.git`
[0m[0m[1m[32m      Loaded[0m 752 security advisories (from /home/qtfkwk/.cargo/advisory-db)
[0m[0m[1m[32m    Updating[0m crates.io index
[0m[0m[1m[32m    Scanning[0m Cargo.lock for vulnerabilities (124 crate dependencies)
```

~~~

## Process `update`, `check`, and `build` targets

~~~text
$ mkrs update check build
# update-toml

```text
$ cargo upgrade -i
    Checking mkrs's dependencies
note: Re-run with `--verbose` to show more dependencies
  latest: 16 packages
```

# update-lock

```text
$ cargo update
    Updating crates.io index
     Locking 0 packages to latest compatible versions
```

# outdated

```text
$ cargo outdated --exit-code=1
All dependencies are up to date, yay!
```

# audit

```text
$ cargo audit
[0m[0m[1m[32m    Fetching[0m advisory database from `https://github.com/RustSec/advisory-db.git`
[0m[0m[1m[32m      Loaded[0m 752 security advisories (from /home/qtfkwk/.cargo/advisory-db)
[0m[0m[1m[32m    Updating[0m crates.io index
[0m[0m[1m[32m    Scanning[0m Cargo.lock for vulnerabilities (124 crate dependencies)
```

# `target/release/mkrs`

```text
$ cargo build --release
   Compiling mkrs v0.23.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.89s
```

~~~

## Serve the documentation

Run `mkrs serve-doc` then open a browser and navigate to <http://localhost:8080>.

~~~text
$ mkrs serve-doc
# serve-doc

```text
$ miniserve -p 8080 target/doc
miniserve v0.29.0
Bound to [::]:8080, 0.0.0.0:8080
Serving path /home/qtfkwk/github.com/qtfkwk/mkrs/target/doc
Available at (non-exhaustive list):
    http://127.0.0.1:8080
    http://192.168.18.14:8080
    http://192.168.122.1:8080
    http://[::1]:8080

Quit by pressing CTRL-C
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

* `target/release/{dirname}`

```
target/release/{dirname}
```

# clippy

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo clippy -- -D clippy::all
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`

```
cargo test
```

# bench

```
cargo bench -q 2>&1 |tee benches/report.txt
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

* `Cargo.lock`
* `Cargo.toml`
* `**/*.rs`
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `**/*.rs`

```
cargo build --release
kapow {0} >{target}
```

# doc

```
cargo doc
```

# serve-doc

```
miniserve -p 8080 target/doc
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
cargo install cargo-audit cargo-edit cargo-outdated cocomo dtg kapow miniserve tokei toml-cli
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

# commit

```bash
set -xeo pipefail
V=$(toml get -r Cargo.toml package.version)
git commit -m "$V"
git tag -a "$V" -m "$V"
```

# publish

```
cargo publish
git push
git push --tags
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
 TOML                    1           27           25            0            2
-------------------------------------------------------------------------------
 Markdown                5         1160            0          856          304
 |- BASH                 3          112           90            6           16
 |- Python               1            1            1            0            0
 (Total)                           1273           91          862          320
-------------------------------------------------------------------------------
 Rust                    1          779          661           36           82
 |- Markdown             1           15            0           15            0
 (Total)                            794          661           51           82
===============================================================================
 Total                   7         1966          686          892          388
===============================================================================

Total Physical Source Lines of Code (SLOC)                    = 686
Development Effort Estimate, Person-Years (Person-Months)     = 0.13 (1.62)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.25 (3.00)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 0.54
Total Estimated Cost to Develop                               = $18,188
  (average salary = $56,286/year, overhead = 2.40)

Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 686
Estimated Cost to Develop  | $18,187.87
Estimated Schedule Effort  | 3.00 months
Estimated People Required  | 0.54

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

## Use with a `.env` file via dotenv

1. Install [`dotenv`]: `cargo install dotenv`.
2. Create a `.env` file with environment variables.
3. Prepend command(s) in your `Makefile.md` recipes with `dotenv `.
4. Run the `mkrs` command.

# Changelog

See [`CHANGELOG.md`] in the [repository].

[`CHANGELOG.md`]: https://github.com/qtfkwk/mkrs/blob/main/CHANGELOG.md
[repository]: https://github.com/qtfkwk/mkrs

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to *compile* any sort of
file; all such commands must be defined in the configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the target(s) that are
being processed.

