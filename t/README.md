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
!run:../target/release/mkrs -C .. -V 2>&1
~~~

~~~text
$ mkrs -h
!run:../target/release/mkrs -C .. -h 2>&1
~~~

# Examples

## List available targets

~~~text
$ mkrs -l
!run:../target/release/mkrs -C .. -l 2>&1
~~~

## List dependencies for `full` target

~~~text
$ mkrs -l full
!run:../target/release/mkrs -C .. -l full 2>&1
~~~

## Dry run

~~~text
$ mkrs -n
!run:../target/release/mkrs -C .. -n 2>&1
~~~

## Process default target

~~~text
$ mkrs
!run:../target/release/mkrs -C .. 2>&1
~~~

## Process `check` target

~~~text
$ mkrs check
!run:../target/release/mkrs -C .. check 2>&1
~~~

## Process `update`, `check`, and `build` targets

~~~text
$ mkrs update check build
!run:../target/release/mkrs -C .. update check build 2>&1
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
!run:../target/release/mkrs -C .. -g rust 2>&1
~~~

**Note:** Save to `Makefile.md` via redirection: `mkrs -g rust >Makefile.md`

## Generate a COCOMO report

~~~text
$ mkrs cocomo
!run:../target/release/mkrs -C .. cocomo 2>&1
~~~

## Use a custom shell program

~~~text
$ mkrs custom
!run:../target/release/mkrs -C .. custom 2>&1
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

