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
    * `{dirname}`: directory name
    * `{name}`: package name from `Cargo.toml`
    * `{target}`: target name
* Targets may use the following variables:
    * `{dirname}`: directory name
    * `{name}`: package name from `Cargo.toml`
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
mkrs 0.25.1
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
cargo clippy -- -D clippy::all -D clippy::pedantic
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
# clippy

```text
$ cargo clippy -- -D clippy::all -D clippy::pedantic
    Checking mkrs v0.25.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.25s
```

# test

```text
$ cargo test
   Compiling mkrs v0.25.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `test` profile [unoptimized + debuginfo] target(s) in 0.30s
     Running unittests src/main.rs (target/debug/deps/mkrs-691fb40b2c6b7352)

running 0 tests

test result: ok. 0 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

```

# `target/release/mkrs`

```text
$ cargo build --release
   Compiling mkrs v0.25.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.54s
```

# doc

```text
$ cargo doc
    Checking unicode-ident v1.0.24
 Documenting unicode-ident v1.0.24
 Documenting bitflags v2.11.0
 Documenting memchr v2.8.0
 Documenting libc v0.2.182
 Documenting linux-raw-sys v0.12.1
 Documenting regex-syntax v0.8.10
 Documenting log v0.4.29
 Documenting anstyle v1.0.13
 Documenting clap_lex v1.0.0
    Checking heck v0.5.0
 Documenting same-file v1.0.6
 Documenting heck v0.5.0
 Documenting strsim v0.11.1
 Documenting crossbeam-utils v0.8.21
 Documenting typenum v1.19.0
 Documenting cfg-if v1.0.4
 Documenting cpufeatures v0.2.17
 Documenting anyhow v1.0.102
 Documenting serde_core v1.0.228
 Documenting either v1.15.0
 Documenting constant_time_eq v0.4.2
 Documenting utf8parse v0.2.2
 Documenting arrayref v0.3.9
 Documenting arrayvec v0.7.6
 Documenting unicode-width v0.2.2
 Documenting winnow v0.7.14
 Documenting anstyle-query v1.1.5
 Documenting colorchoice v1.0.4
 Documenting is_terminal_polyfill v1.70.2
 Documenting owo-colors v4.3.0
 Documenting hashbrown v0.16.1
 Documenting pulldown-cmark-escape v0.11.0
 Documenting equivalent v1.0.2
 Documenting shlex v1.3.0
 Documenting toml_writer v1.0.6+spec-1.1.0
    Checking proc-macro2 v1.0.106
 Documenting unicase v2.9.0
 Documenting lazy_static v1.5.0
 Documenting proc-macro2 v1.0.106
 Documenting glob v0.3.3
    Checking quote v1.0.44
 Documenting anstyle-parse v1.0.0
    Checking syn v2.0.117
 Documenting walkdir v2.5.0
 Documenting notify-types v2.1.0
 Documenting blake3 v1.8.3
 Documenting rustix v1.1.4
 Documenting crossbeam-epoch v0.9.18
 Documenting getopts v0.2.24
 Documenting aho-corasick v1.1.4
 Documenting bstr v1.12.1
 Documenting indexmap v2.13.0
 Documenting quote v1.0.44
 Documenting anstream v1.0.0
 Documenting toml_parser v1.0.9+spec-1.1.0
 Documenting inotify-sys v0.1.5
 Documenting mio v1.1.1
 Documenting errno v0.3.14
 Documenting dirs v1.0.5
 Documenting num_cpus v1.17.0
 Documenting generic-array v0.14.7
 Documenting serde_spanned v1.0.4
 Documenting toml_datetime v1.0.0+spec-1.1.0
 Documenting terminal_size v0.4.3
 Documenting crossbeam-deque v0.8.6
 Documenting pulldown-cmark v0.13.1
 Documenting regex-automata v0.4.14
 Documenting syn v2.0.117
 Documenting inotify v0.11.0
 Documenting pager2 v0.6.4
 Documenting dep-graph v0.2.0
 Documenting block-buffer v0.10.4
 Documenting crypto-common v0.1.7
 Documenting toml v1.0.3+spec-1.1.0
 Documenting clap_builder v4.5.60
 Documenting rayon-core v1.13.0
 Documenting globset v0.4.18
 Documenting regex v1.12.3
 Documenting clap_derive v4.5.55
 Documenting thiserror-impl v1.0.69
 Documenting notify v8.2.0
 Documenting digest v0.10.7
 Documenting rayon v1.11.0
 Documenting ignore v0.4.25
 Documenting thiserror v1.0.69
 Documenting clap v4.5.60
 Documenting sha2 v0.10.9
 Documenting ignore-check v0.3.3
 Documenting pwd v1.4.0
 Documenting clap-cargo v0.18.3
 Documenting expanduser v1.2.2
 Documenting fhc v0.11.4
 Documenting sprint v0.12.4
 Documenting mkrs v0.25.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 41.11s
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
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 939 security advisories (from /home/qtfkwk/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (130 crate dependencies)
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
     Locking 0 packages to latest Rust 1.93.1 compatible versions
note: pass `--verbose` to see 1 unchanged dependencies behind latest
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
      Loaded 939 security advisories (from /home/qtfkwk/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (130 crate dependencies)
```

# `target/release/mkrs`

```text
$ cargo build --release
   Compiling mkrs v0.25.1 (/media/sda1/backup-20250317/home/nick/github.com/qtfkwk/mkrs)
    Finished `release` profile [optimized] target(s) in 1.55s
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
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Language              Files        Lines         Code     Comments       Blanks
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 TOML                      1           27           25            0            2
─────────────────────────────────────────────────────────────────────────────────
 Markdown                  5         1272            0          968          304
 |- BASH                   3          112           90            6           16
 |- Python                 1            1            1            0            0
 (Total)                             1385           91          974          320
─────────────────────────────────────────────────────────────────────────────────
 Rust                      1          801          670           48           83
 |- Markdown               1           16            0           16            0
 (Total)                              817          670           64           83
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
 Total                     7         2229          786         1038          405
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

Total Physical Source Lines of Code (SLOC)                    = 695
Development Effort Estimate, Person-Years (Person-Months)     = 0.14 (1.64)
  (Basic COCOMO model, Person-Months = 2.40*(KSLOC**1.05)*1.00)
Schedule Estimate, Years (Months)                             = 0.25 (3.02)
  (Basic COCOMO model, Months = 2.50*(person-months**0.38))
Estimated Average Number of Developers (Effort/Schedule)      = 0.54
Total Estimated Cost to Develop                               = $18,439
  (average salary = $56,286/year, overhead = 2.40)

Description                | Value
---------------------------|---------------------------------
Total Source Lines of Code | 695
Estimated Cost to Develop  | $18,438.50
Estimated Schedule Effort  | 3.02 months
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

