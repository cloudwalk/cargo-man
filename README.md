# Cargo-man


[![Crates.io](https://img.shields.io/crates/v/cargo-man?style=flat-square)](https://crates.io/crates/cargo-man)
[![Crates.io](https://img.shields.io/crates/d/cargo-man?style=flat-square)](https://crates.io/crates/cargo-man)
[![License](https://img.shields.io/badge/license-MIT-blue?style=flat-square)](https://github.com/cloudwalk/cargo-man/blob/master/LICENSE-MIT)
[![Contributors](https://img.shields.io/github/contributors/cloudwalk/cargo-man?style=flat-square)](https://github.com/cloudwalk/cargo-man/graphs/contributors)

The man is a simples CLI sub-command from cargo, that helps you manage your `Cargo.toml` file.

## Install:

To install the sub-command you just need to execute the following command:
```shell
cargo install cargo-man
```

## Usage:

To use the CLI you must inform the fields you want it to return or change using the flag `--field` or `-f`:

```shell
cargo man -f version
0.1.0
```

If you want to attribute a new value just use the flag `--set` or `-s`

```shell
cargo man -f version -s 1.0.0
1.0.0
```

By default, cargo-man looks at the `package` table, but you can change that by anything that you want it

```shell
cargo man -t dependencies -f something
1.0.0
```