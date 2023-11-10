# build

* clippy
* `README.md`

```
cargo build --release
```

# `README.md`

* `t/README.md`
* `Cargo.toml`
* `CHANGELOG.md`
* `src/main.rs`

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
  
