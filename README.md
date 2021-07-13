# Cargo-man
The man is a simples CLI sub-command from cargo, that helps you manage your `Cargo.toml` file.

> In this first version we are just managing the `package` table from the manifest, in future versions we are going to support all tables.

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
