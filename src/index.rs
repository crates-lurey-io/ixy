//! Tools for mapping between 2D grid coordinates and 1D memory indices.
//!
//! This module provides a standard interface, [`Layout`], for defining how a 2D grid is stored in
//! linear memory. It includes the three most common memory layouts:
//!
//! * [`RowMajor`]: Elements in the same row are contiguous in memory.
//! * [`ColMajor`]: Elements in the same column are contiguous in memory.
//! * [`Block`]: Elements are stored in fixed-size blocks, which can improve cache locality.
//!
//! # Examples
//!
//! ```rust
//! use ixy::{Pos, index::{RowMajor, ColMajor, Layout}};
//!
//! let pos = Pos::new(2, 3);
//! let width = 5;
//!
//! // Convert a 2D position to a 1D index in row-major layout
//! let index = RowMajor::to_1d(pos, width);
//! assert_eq!(index, 17); // 3 * 5 + 2
//!
//! // Convert a 1D index back to a 2D position in row-major layout
//! let pos: Pos<i32> = RowMajor::to_2d(index, width);
//! assert_eq!(pos, Pos::new(2, 3));
//! ```

use crate::{Pos, Rect, int::Int};

/// A mapping between a 2-dimensional array and a linear memory layout.
#[allow(private_bounds)]
pub trait Layout: Sized + crate::internal::Sealed {
    /// Converts a 2-dimensional position to a linear memory index.
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> usize;

    /// Converts a linear memory index to a 2-dimensional position.
    fn to_2d<T: Int>(index: usize, width: usize) -> Pos<T>;

    /// Creates an iterator of the given bounds, yielding positions in the order defined.
    fn positions<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>>;

    /// Returns the layout as a type erased enum.
    fn as_any() -> AnyLayout;
}

/// Continuous memory with the first row at the lowest address.
pub enum RowMajor {}

impl crate::internal::Sealed for RowMajor {}

impl Layout for RowMajor {
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> usize {
        pos.y.to_usize() * width + pos.x.to_usize()
    }

    fn to_2d<T: Int>(index: usize, width: usize) -> Pos<T> {
        Pos {
            x: T::from_usize(index % width),
            y: T::from_usize(index / width),
        }
    }

    fn positions<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        let mut current = bounds.top_left();
        core::iter::from_fn(move || {
            if current.y >= bounds.bottom() {
                return None;
            }
            let pos = current;
            current.x += T::ONE;

            if current.x >= bounds.right() {
                current.x = bounds.left();
                current.y += T::ONE;
            }

            Some(pos)
        })
    }

    fn as_any() -> AnyLayout {
        AnyLayout::RowMajor
    }
}

/// Continuous memory with the first column at the lowest address.
pub enum ColMajor {}

impl crate::internal::Sealed for ColMajor {}

impl Layout for ColMajor {
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> usize {
        pos.x.to_usize() * width + pos.y.to_usize()
    }

    fn to_2d<T: Int>(index: usize, width: usize) -> Pos<T> {
        Pos {
            x: T::from_usize(index / width),
            y: T::from_usize(index % width),
        }
    }

    fn positions<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        let mut current = bounds.top_left();
        core::iter::from_fn(move || {
            if current.x >= bounds.right() {
                return None;
            }
            let pos = current;
            current.y += T::ONE;

            if current.y >= bounds.bottom() {
                current.y = bounds.top();
                current.x += T::ONE;
            }

            Some(pos)
        })
    }

    fn as_any() -> AnyLayout {
        AnyLayout::ColMajor
    }
}

/// A cache-friendly block layout, which stores data in continuous fixed-size (`W x H`) blocks.
///
/// For example, a `Block<2, 2>` layout that stores two elements would look like this:
///
/// ```text
/// 00, 01, 04, 05
/// 02, 03, 06, 07
/// ```
pub enum Block<const W: usize, const H: usize> {}

impl<const W: usize, const H: usize> Block<W, H> {
    /// Returns an iterator over blocks of the size `W x H` within the given rectangle.
    ///
    /// It yields `Rect<T>` instances representing each block.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn blocks<T: Int>(rect: Rect<T>) -> impl Iterator<Item = Rect<T>> {
        // TODO: Implement.
        core::iter::empty()
    }
}

impl<const W: usize, const H: usize> crate::internal::Sealed for Block<W, H> {}

impl<const W: usize, const H: usize> Layout for Block<W, H> {
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> usize {
        let block_x = pos.x.to_usize() / W;
        let block_y = pos.y.to_usize() / H;
        let offset_x = pos.x.to_usize() % W;
        let offset_y = pos.y.to_usize() % H;

        (block_y * (width / W) + block_x) * (W * H) + (offset_y * W + offset_x)
    }

    fn to_2d<T: Int>(index: usize, width: usize) -> Pos<T> {
        let block_size = W * H;
        let block_index = index / block_size;
        let offset_index = index % block_size;

        let block_x = block_index % (width / W);
        let block_y = block_index / (width / W);

        let offset_x = offset_index % W;
        let offset_y = offset_index / W;

        Pos {
            x: T::from_usize(block_x * W + offset_x),
            y: T::from_usize(block_y * H + offset_y),
        }
    }

    #[allow(clippy::similar_names)]
    fn positions<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        // TODO: Implement.
        core::iter::empty()
    }

    fn as_any() -> AnyLayout {
        AnyLayout::Block {
            width: W,
            height: H,
        }
    }
}

/// Type erased layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum AnyLayout {
    /// Row-major layout.
    RowMajor,

    /// Column-major layout.
    ColMajor,

    /// Block layout.
    Block { width: usize, height: usize },
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::pos;
    use alloc::vec::Vec;

    #[test]
    fn to_row_major() {
        let pos = Pos::new(2, 3);
        let width = 5;

        let index = RowMajor::to_1d(pos, width);
        assert_eq!(index, 17); // 3 * 5 + 2
    }

    #[test]
    fn to_col_major() {
        let pos = Pos::new(2, 3);
        let width = 5;

        let index = ColMajor::to_1d(pos, width);
        assert_eq!(index, 13); // 2 * 5 + 3
    }

    #[test]
    fn to_block() {
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(0, 0), 4), 0);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(1, 0), 4), 1);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(0, 1), 4), 2);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(1, 1), 4), 3);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(2, 0), 4), 4);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(3, 0), 4), 5);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(2, 1), 4), 6);
        assert_eq!(Block::<2, 2>::to_1d(Pos::new(3, 1), 4), 7);
    }

    #[test]
    fn from_row_major() {
        let index = 17;
        let width = 5;

        let pos: Pos<i32> = RowMajor::to_2d(index, width);
        assert_eq!(pos.x, 2); // 17 % 5
        assert_eq!(pos.y, 3); // 17 / 5
    }

    #[test]
    fn from_col_major() {
        let index = 13;
        let width = 5;

        let pos: Pos<i32> = ColMajor::to_2d(index, width);
        assert_eq!(pos.x, 2); // 13 / 5
        assert_eq!(pos.y, 3); // 13 % 5
    }

    #[test]
    fn from_block() {
        assert_eq!(Block::<2, 2>::to_2d(0, 4), Pos::new(0, 0));
        assert_eq!(Block::<2, 2>::to_2d(1, 4), Pos::new(1, 0));
        assert_eq!(Block::<2, 2>::to_2d(2, 4), Pos::new(0, 1));
        assert_eq!(Block::<2, 2>::to_2d(3, 4), Pos::new(1, 1));
        assert_eq!(Block::<2, 2>::to_2d(4, 4), Pos::new(2, 0));
        assert_eq!(Block::<2, 2>::to_2d(5, 4), Pos::new(3, 0));
        assert_eq!(Block::<2, 2>::to_2d(6, 4), Pos::new(2, 1));
        assert_eq!(Block::<2, 2>::to_2d(7, 4), Pos::new(3, 1));
    }

    #[test]
    fn iter_row_major() {
        let bounds = Rect::from_ltrb(0, 0, 3, 2).unwrap();
        let positions: Vec<_> = RowMajor::positions(bounds).collect();
        assert_eq!(
            positions,
            &[
                pos!(0, 0),
                pos!(1, 0),
                pos!(2, 0),
                pos!(0, 1),
                pos!(1, 1),
                pos!(2, 1)
            ]
        );
    }

    #[test]
    fn iter_col_major() {
        let bounds = Rect::from_ltrb(0, 0, 3, 2).unwrap();
        let positions: Vec<_> = ColMajor::positions(bounds).collect();
        assert_eq!(
            positions,
            &[
                pos!(0, 0),
                pos!(0, 1),
                pos!(1, 0),
                pos!(1, 1),
                pos!(2, 0),
                pos!(2, 1)
            ]
        );
    }

    #[test]
    fn iter_block() {
        let bounds = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = Block::<2, 2>::positions(bounds).collect();
        assert_eq!(
            positions,
            &[
                pos!(0, 0),
                pos!(1, 0),
                pos!(0, 1),
                pos!(1, 1),
                pos!(2, 0),
                pos!(3, 0),
                pos!(2, 1),
                pos!(3, 1),
            ]
        );
    }

    #[test]
    fn any_layout() {
        assert_eq!(RowMajor::as_any(), AnyLayout::RowMajor);
        assert_eq!(ColMajor::as_any(), AnyLayout::ColMajor);
        assert_eq!(
            Block::<2, 2>::as_any(),
            AnyLayout::Block {
                width: 2,
                height: 2
            }
        );
    }
}
