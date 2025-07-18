# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `Layout::iter_pos` and associated methods take ownership of `Rect`

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
