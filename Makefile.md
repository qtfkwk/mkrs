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
* `src/**/*.rs`

```
cargo clippy -- -D clippy::all -D clippy::pedantic
```

# test

* `Cargo.lock`
* `Cargo.toml`
* `src/**/*.rs`

```
cargo test
```

# build

* `target/release/{dirname}`

# `target/release/{dirname}`

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
* `img/crates.png`

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

# fail

```
echo This command runs

#echo Ignore this command

echo This command runs too but the next one fails

exit 1

echo This does not run because the prior command failed
```

# `nonexistent`

```
echo
```

# custom

```python
print("This is a custom recipe in Python.")
```

# `*.png`

* `*.gv`

```
dot -Tpng {0} >{target}
```
  
