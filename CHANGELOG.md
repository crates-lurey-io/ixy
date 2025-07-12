# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- `grid::impls::get_mut_from*` for mutable references to elements in a grid
- `grid::iter::*`, for accessing elements lazily in an iterable fashion
- `GridRead[Unchecked]::iter_*`, for reading a block of elements lazily
- `GridReadMut[Unchecked]` for mutable references, implemented for `GridBuf`
- `Layout::iter_pos`, for yielding positions lazily in an iterable fasion
- `Rect::contains[_{pos|rect}]`, `Rect::iter_pos_{row|col}`

### Changed

- Renamed `Grid*Ext` to `GridSubView*`, changed the API to match `GridView::*`
- `GridSubView*` now require a `Rect<usize>` instead of `impl Into<Rect<usize>>`

## [0.1.0] - 2025-07-10

### Added

- Initial release.

