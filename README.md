Cargo listdeps
==============

A utility tool for printing out dependency license information.

For a cargo workspace `.toml` file, it will print out information about all transitive dependencies and filter out workspace members.

## Install

```bash
cargo install cargo-listdeps
```

## Use

```bash
cargo listdeps path-to-Cargo.toml
```