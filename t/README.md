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
!run:../target/release/mkrs -f ../Makefile.md -V 2>&1
~~~

~~~text
$ mkrs -h
!run:../target/release/mkrs -f ../Makefile.md -h 2>&1
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
!run:../target/release/mkrs -f ../Makefile.md -n 2>&1
~~~

## Process default target

~~~text
$ mkrs
!run:../target/release/mkrs -f ../Makefile.md 2>&1
~~~

## Process `check` target

~~~text
$ mkrs check
!run:../target/release/mkrs -f ../Makefile.md check 2>&1
~~~

## Process `update`, `check`, and `build` targets

~~~text
$ mkrs update check build
!run:../target/release/mkrs -f ../Makefile.md update check build 2>&1
~~~

## Generate a default Makefile.md for a Rust project

~~~text
$ mkrs -g rust
!run:../target/release/mkrs -f ../Makefile.md -g rust 2>&1
~~~

**Note:** Save to `Makefile.md` via redirection: `mkrs -g rust >Makefile.md`

## Generate a COCOMO report

~~~text
$ mkrs cocomo
!run:../target/release/mkrs -f ../Makefile.md cocomo 2>&1
~~~

## Use a custom shell program

~~~text
$ mkrs custom
!run:../target/release/mkrs -f ../Makefile.md custom 2>&1
~~~

## Use with a `.env` file via dotenv

1. Install [`dotenv`]: `cargo install dotenv`.
2. Create a `.env` file with environment variables.
3. Prepend command(s) in your `Makefile.md` recipes with `dotenv `.
4. Run the `mkrs` command.

!inc:../CHANGELOG.md

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to
*compile* any sort of file; all such commands must be defined in the
configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the
target(s) that are being processed.

