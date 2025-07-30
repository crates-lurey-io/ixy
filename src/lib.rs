//! A terse, no-std crate for 2D integer geometry.
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

#![no_std]

pub mod index;
pub mod int;
pub mod ops;

pub(crate) mod internal;

mod pos;
pub use pos::*;

mod rect;
pub use rect::*;

mod size;
pub use size::*;
