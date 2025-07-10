//! A terse, no-std crate for 2D integer geometry.

#![no_std]

pub mod grid;

pub mod index;
pub use index::Index;

pub mod int;

pub(crate) mod internal;

mod pos;
pub use pos::*;

mod rect;
pub use rect::*;

mod size;
pub use size::*;
