# KVStore Example App

## Important Note

This project contains a build script (in `build.rs`) which is responsible for
installing the `abci-cli` in the project root. As a result, the first `cargo
build` should take a considerable amount of time (since the comet repo needs to
be cloned.) However every subsequent build should be normal.

## Getting started

- Make sure you have Rust installed.
- Make sure you have Go installed.

## How to use

In one shell, run `cargo run --release` (release is not strictly necessary of
course.)

Run `abci-cli console` to more easily run `abci` commands.

1. Run `deliver_tx "foo=bar"` to insert the key value pair `("foo", "bar")`
2. To check that the key value pair was correctly inserted, run `query "foo"`.
   You should get the following result.

   ```
    -> code: OK
    -> log: value exists
    -> height: 1
    -> key: foo
    -> key.hex: 666F6F
    -> value: bar
    -> value.hex: 626172
   ```
