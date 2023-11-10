# About

Build automation tool

* Inspired by [make]
* No automatic targets[^one]
* Minimalist functionality; maximalist readability
* Configuration is a simple Markdown file named [`Makefile.md`]
* Output is colorized Markdown
* Processes the target(s) specified or if none, processes the first target
* Commands are run independently via `sh -c` by default, or if script mode is
  enabled via `-s`, entire target recipes are run via `bash -eo pipefail`
* If any command fails (exits with a non-zero code), processing halts
  immediately
* Verbosity levels: `-v`: add `-x` to `bash` command in script mode, `-vv`:
  print up to date targets, `-vvv`: show configuration
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

!inc:../CHANGELOG.md

---

[^one]: Unlike [make], mkrs does not have any built-in knowledge about how to
*compile* any sort of file; all such commands must be defined in the
configuration file.

[^two]: A target of either sort is only processed if it is a dependency of the
target(s) that are being processed.

