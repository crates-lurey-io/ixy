# ixy

A minimal, no-std compatible crate for 2D integer geometry

[![Test](https://github.com/crates-lurey-io/ixy/actions/workflows/test.yml/badge.svg)](https://github.com/crates-lurey-io/ixy/actions/workflows/test.yml)
[![Crates.io Version](https://img.shields.io/crates/v/ixy)](https://crates.io/crates/ixy)
[![codecov](https://codecov.io/gh/crates-lurey-io/ixy/graph/badge.svg?token=Z3VUWA3WYY)](https://codecov.io/gh/crates-lurey-io/ixy)

## Contributing

This project uses [`just`][] to run commands the same way as the CI:

- `cargo just check` to check formatting and lints.
- `cargo just coverage` to generate and preview code coverage.
- `cargo just doc` to generate and preview docs.
- `cargo just semver-checks` to verify no accidental breaking changes.
- `cargo just test` to run tests.

[`just`]: https://crates.io/crates/just

For a full list of commands, see the [`Justfile`](./Justfile).

## Releasing

1. Update the version in `Cargo.toml` and commit.
2. Push a tag matching the version (e.g. `v0.6.0-alpha.4`).
3. The [`publish`](.github/workflows/publish.yml) workflow handles the rest:
   - Validates the tag matches `Cargo.toml`
   - Runs checks, tests, and semver-checks
   - Publishes to [crates.io](https://crates.io/crates/ixy) via trusted publishing
   - Creates a [GitHub Release](https://github.com/crates-lurey-io/ixy/releases) with changelog

## Inspiration

- [`tinygeom2d`](https://github.com/ttalvitie/tinygeom2d)
