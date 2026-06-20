# Agent Guidelines for ixy

## Rust API Guidelines

Prefer following the [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/checklist.html).

Key points:
- No trait bounds on struct definitions (`C-STRUCT-BOUNDS`)
- Sealed traits for downstream-safe abstractions (`C-SEALED`)
- `no_std` compatible, verified in CI
- `forbid(unsafe_code)` — no unsafe blocks in production code
- Optional `serde` feature via `cfg_attr`
- Dual-licensed `MIT OR Apache-2.0`
- Keep `CHANGELOG.md` up to date

## Code style

- `cargo just check` before committing (lint + format)
- `cargo just test-all` to run all tests
- `cargo just semver-checks` to verify no accidental breaking changes
- All public items must have doc examples
- `#[must_use]` on all methods returning a value
