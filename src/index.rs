//! Tools for mapping between 2D grid coordinates and 1D memory indices.
//!
//! This module provides a standard interface, [`Layout`], for defining how a 2D grid is stored in
//! linear memory. It includes the two most common memory layouts:
//!
//! * [`RowMajor`]: Elements in the same row are contiguous in memory.
//! * [`ColMajor`]: Elements in the same column are contiguous in memory.
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

use core::marker::PhantomData;

use crate::{Pos, Rect, int::Int};

/// A mapping between a 2-dimensional array and a linear memory layout.
#[allow(private_bounds)]
pub trait Layout: Sized + crate::internal::Sealed {
    /// Converts a 2-dimensional position to a linear memory index.
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> usize;

    /// Converts a linear memory index to a 2-dimensional position.
    fn to_2d<T: Int>(index: usize, width: usize) -> Pos<T>;

    /// Creates an iterator of the given bounds, yielding positions in the order defined.
    fn iter_pos<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>>;
}

/// An iterator over a 2-dimensional bounding rectangle, yielding positions.
pub struct IterPos<T: Int, L: Layout> {
    bounds: Rect<T>,
    current: Pos<T>,
    layout: PhantomData<L>,
}

impl<T> Iterator for IterPos<T, RowMajor>
where
    T: Int,
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.bounds.bottom() {
            return None;
        }

        let pos = self.current;
        self.current.x += T::ONE;

        if self.current.x >= self.bounds.right() {
            self.current.x = self.bounds.left();
            self.current.y += T::ONE;
        }

        Some(pos)
    }
}

impl<T> Iterator for IterPos<T, ColMajor>
where
    T: Int,
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x >= self.bounds.right() {
            return None;
        }

        let pos = self.current;
        self.current.y += T::ONE;

        if self.current.y >= self.bounds.bottom() {
            self.current.y = self.bounds.top();
            self.current.x += T::ONE;
        }

        Some(pos)
    }
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

    fn iter_pos<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        IterPos {
            bounds,
            current: Pos::new(bounds.left(), bounds.top()),
            layout: PhantomData::<Self>,
        }
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

    fn iter_pos<T: Int>(bounds: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        IterPos {
            bounds,
            current: Pos::new(bounds.left(), bounds.top()),
            layout: PhantomData::<Self>,
        }
    }
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
    fn iter_row_major() {
        let bounds = Rect::from_ltrb(0, 0, 3, 2).unwrap();
        let positions: Vec<_> = RowMajor::iter_pos(bounds).collect();
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
        let positions: Vec<_> = ColMajor::iter_pos(bounds).collect();
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
}
