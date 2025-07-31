# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `Rect` now implements `Sub|SubAssign<Pos<T>>` for translation

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
