//! A terse, no-std crate for 2D integer geometry.

#![no_std]

pub mod index;
pub mod int;

pub(crate) mod internal;

mod pos;
pub use pos::*;

mod rect;
pub use rect::*;

mod size;
pub use size::*;
