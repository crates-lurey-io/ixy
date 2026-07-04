//! A terse, no-std crate for 2D integer geometry.
//!
//! ## Coordinate system
//!
//! All types in this crate use a _y-down_ coordinate system, where the origin `(0, 0)` is at the
//! top-left and `y` increases downward:
//!
//! ```txt
//! +---------→ x
//! |
//! |
//! ↓
//! y
//! ```
//!
//! This is the natural convention for screens, terminals, images, and most 2D game grids, and
//! matches the row-major ordering used by [`Pos`]'s [`Ord`] implementation and by the [`layout`]
//! module's default traversal order.
//!
//! ## Related crates
//!
//! This crate deliberately stays allocation-free and does not provide an owning grid type. For a
//! `Vec`-backed grid built on top of [`layout`]'s traversal and indexing abstractions, see the
//! sibling crate [`grixy`](https://docs.rs/grixy).
//!
//! ## Examples
//!
//! ```rust
//! use ixy::{Pos, Rect};
//!
//! let pos = Pos::new(10, 20);
//! let rect = Rect::from_ltwh(0, 0, 100, 200);
//!
//! assert_eq!(pos.x, 10);
//! assert_eq!(pos.y, 20);
//! assert_eq!(rect.left(), 0);
//! assert_eq!(rect.top(), 0);
//! assert_eq!(rect.width(), 100);
//! assert_eq!(rect.height(), 200);
//! assert_eq!(rect.right(), 100);
//! assert_eq!(rect.bottom(), 200);
//! assert!(rect.contains_pos(pos));
//! assert!(!rect.contains_pos(Pos::new(150, 250)));
//! ```

#![cfg_attr(docsrs, feature(doc_cfg))]
#![no_std]
#![forbid(unsafe_code)]

pub mod int;
pub mod layout;
pub mod ops;

pub(crate) mod internal;

mod pos;
pub use pos::*;

mod rect;
pub use rect::*;

mod size;
pub use size::*;
