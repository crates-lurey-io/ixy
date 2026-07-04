# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.6.0] - 2026-07-04

Pre-1.0 API and architecture cleanup pass: closes gaps found in a full crate review and
comparison against `glam`, `euclid`, `vek`, and `bracket-lib`. All changes are breaking.

### Added

- `Rect::union()`, `Rect::inflate()`, `Rect::shrink()`, `Rect::center()`, `Rect::overlaps()`
- `impl IntoIterator for Rect<T>` — supports `for pos in rect`
- `Pos::min()`, `Pos::max()`, `Pos::clamp()`, `Pos::abs()`, `Pos::dot()`, `Pos::splat()`
- `Pos::ZERO` — alias for `Pos::ORIGIN`
- `impl Rem|RemAssign<T>` and `impl Rem|RemAssign<Pos<T>>` for `Pos<T>`
- `Pos::try_cast::<U>()` — replaces `TryFromPos`/`TryIntoPos`
- Scalar-on-left multiplication: `n * pos`, not just `pos * n`
- `ops::distance::manhattan()` and `ops::distance::chebyshev()`
- `ops::line::bresenham()` — pixel-perfect line algorithm, alongside the existing `vector()`
- `impl Display` and `impl core::error::Error` for `RectError` and `TryFromPosError`
- `From<Size<T>> for Pos<T>`
- Crate-level docs: explicit y-down coordinate convention, and a pointer to
  [`grixy`](https://docs.rs/grixy) for an owning, allocation-backed grid type

### Changed

- **`Size<T>` is now generic over `Int`** (was hardcoded to `usize`), matching `Pos<T>` and
  `Rect<T>`; defaults to `Size<usize>` for source compatibility with most call sites
- **`HasSize` is now generic over `Int`** (`HasSize<T = usize>`) to match `Size<T>`
- **`Rect::new()`, `Rect::from_ltwh()`, `Rect::from_tl_size()`** now take `T` uniformly for
  width/height instead of mixing `T` position with `usize` dimensions
- **`layout::Traversal` renamed to `layout::Layout`**
- **`layout::Linear` renamed to `layout::LinearLayout`**
- **`LinearLayout::pos_to_index()` / `index_to_pos()`** parameter renamed from `width` to
  `stride` (was misleading for column-major and block layouts, where it isn't a row width)
- Added `#[must_use]` to public methods that were silently missing it — `must_use_candidate`
  and `return_self_not_must_use` do not fire inside generic `impl<T: Int>` blocks, so most of
  the public API was previously unprotected by the crate's own `deny`-level lint config

### Fixed

- `ExactSizeIterator::len()` on row-major, column-major, and block position/block iterators
  undercounted remaining items for partially-consumed rows/columns
- Doc typo ("thhis" → "this") in `Pos::normalized_approx`

### Removed

- `TryFromPos` / `TryIntoPos` traits — use `Pos::try_cast::<U>()` instead

### Chores (pre-stable pass)

- `missing_docs`, `unreachable_pub`, and `unused_qualifications` lints promoted from `warn` to
  `deny`, and `unsafe_code = "forbid"` added to `[lints.rust]` for parity with the `#![forbid]`
  attribute already in `lib.rs` and with sibling crates' `Cargo.toml` lint configuration.
- `just semver-checks` no longer hardcodes a stale baseline version (previously `0.5.7`); lets
  `cargo-semver-checks` auto-select the latest published release, matching the fix already applied
  in `grixy`.
- Documented the unsigned-underflow panic behavior of `Rect::inflate()`/`Rect::shrink()` when the
  deltas exceed the rectangle's existing position/size.

## [0.6.0-alpha.8] - 2026-06-25

### Added

- `Clone` and `Copy` impls for layout ZSTs: `RowMajor`, `ColumnMajor`, `Block`

## [0.6.0-alpha.7] - 2026-06-19

Major redesign of `Rect` internal storage and API cleanup, laying groundwork for v1.0.

### Added

- `Pos::cmp_lexicographic()` — x-primary ordering (replaces old `Ord` behavior)
- `Rect::new(x, y, width, height)` — primary constructor matching game framework convention
- `Rect::from_tl_size(Pos, Size)` — renamed from `Rect::new(Pos, Size)`
- `Rect::width_usize()` / `Rect::height_usize()` — for indexing use
- `Pos::cmp_row_major()` — y-primary comparison for grid iteration
- `impl Display for Size` — formats as `10×20`
- Type aliases: `Pos16`, `PosI` for positions, `Rect16`, `RectI` for rectangles
- Optional `serde` feature (`serde = ["dep:serde"]`)
- `cargo-semver-checks` in CI
- `Block` layout, which stores data in continuous fixed-size (`W x H`) blocks
- `AGENTS.md` — guidelines for LLM contributors

### Changed

- **`Pos::Ord` is now row-major** (y-primary) instead of lexicographic (x-primary); use `cmp_lexicographic()` for the old behavior
- **`Rect` internal storage changed** from `(l, t, r, b)` to `(x, y, w, h)` — all fields are `T`
- `Rect::width()` and `Rect::height()` now return `T` (was `usize`)
- `Rect::from_ltrb_unchecked()` is no longer `unsafe` — uses `debug_assert!`
- Removed `T: Int` bound from `Pos` and `Rect` struct definitions (`C-STRUCT-BOUNDS`)
- `Rect::new(Pos, Size)` renamed to `Rect::from_tl_size(Pos, Size)`
- `Layout` has been split into 2 traits: 
  - `Layout` for iterating over positions, rectangles, and elements
  - `Linear` for an (optional) optimization of linear-aligned data
- Layouts are now `&self` instead of static-only, allowing for dynamic dispatch
- License changed to `MIT OR Apache-2.0` (was `MIT`)
- Lint configuration aligned with rg: added `rust.nursery`, `must_use_candidate`, more

### Removed

- `unsafe` code from production codebase (`forbid(unsafe_code)`)
- `AsRef` / `AsMut` impls on `Pos` (depended on `unsafe` pointer casts)
- `Rect::iter_pos` and related methods; use the layout traits instead
- `.gemini/GEMINI.md` — replaced by `AGENTS.md`

## [0.5.7] - 2025-08-01

### Added

- `Rect` now implements `Sub|SubAssign<Pos<T>>` for translation
- `Pos` now implements `Mul|MulAssign|Div|DivAssign<Pos<T>>` for scaling

## [0.5.6] - 2025-07-30

### Added

- `Int` now supports:
  - `Shl<u32, Output = Self>` and `ShlAssign<u32>`
  - `Shr<u32, Output = Self>` and `ShrAssign<u32>`
  - `abs` method
  - `trailing_zeros` method
- Added `ixy::ops::{distance, line}` functions
- Added `Pos::normalized_approx` method for approximating a direction vector

## [0.5.5] - 2025-07-30

### Added

- `AnyLayout` and `Layout::as_any` for runtime comparisons of layouts

## [0.5.4] - 2025-07-30

### Changed

- Reverted back to `0.5.2`.

## [0.5.3] - 2025-07-30

_**⚠️ Yanked**: Replaced by `0.5.4` to avoid a breaking change._

### Changed

- `RowMajor` and `ColMajor` are back to variantless enums
- The struct `LayoutKind` was added, which provides dynamic dispatch for layouts

## [0.5.2] - 2025-07-30

_**⚠️ Yanked**: Replaced by `0.5.3` to avoid a breaking change._

### Changed

- `RowMajor` and `ColMajor` now are normal structs with useful traits

## [0.5.1] - 2025-07-29

### Added

- `Size::{add,sub,mul,div}` methods for `Size` arithmetic

## [0.5.0] - 2025-07-19

### Removed

- `Index`
- `Layout::{to_1d|to_2d}` now return (and use) `usize` instead of `Index`
- `Layout::IS_ROW_MAJOR`

## [0.4.0] - 2025-07-18

### Removed

- `Rect::from_ltwh_unsigned`
- `int::UnsignedInt`, which is now unused

## [0.3.0] - 2025-07-18

### Added

- `Rect::intersect`
- `Add<Pos<T>>` and `AddAssign<Pos<T>>` for `Rect<T>`

### Changed

- `Layout::iter_pos` and associated methods take ownership of `Rect`
- `Rect::iter_pos_*` is now `::into_iter_*`, and consumes `Rect`
- `Rect::from_ltwh` now takes `usize` for `w` and `h` and is infallible
- `Size` is now always `usize`-based dimensions, removing the `Int` generic

### Removed

- `IntoSize` in favor of `TryFrom<Pos<T>>` for `Size`

## [0.2.0] - 2025-07-12

### Added

- `Layout::iter_pos`, for yielding positions lazily in an iterable fasion
- `Rect::contains[_{pos|rect}]`, `Rect::iter_pos_{row|col}`

### Removed

- All `Grid*` types, and the `ixy::grid` sub-module
- Unused features (`alloc`) that were never needed

## [0.1.0] - 2025-07-10

### Added

- Initial release
