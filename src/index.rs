use core::marker::PhantomData;

use crate::{Pos, int::Int};

/// A linear memory index that can be used to access elements in a 2-dimensional array.
#[repr(transparent)]
pub struct Index<T: Int, L: Layout = RowMajor> {
    pub index: usize,
    _pos_type: PhantomData<T>,
    _layout: PhantomData<L>,
}

impl<T: Int, L: Layout> Index<T, L> {
    /// Wraps a `usize` index into an `Index`.
    ///
    /// The generics `T` and `L` are used to specify the integer type and layout of the index.
    #[must_use]
    pub const fn new(index: usize) -> Self {
        Self {
            index,
            _pos_type: PhantomData,
            _layout: PhantomData,
        }
    }
}

/// A mapping between a 2-dimensional array and a linear memory layout.
#[allow(private_bounds)]
pub trait Layout: Sized + crate::internal::Sealed {
    /// Whether the layout is row-major.
    const IS_ROW_MAJOR: bool;

    /// Converts a 2-dimensional position to a linear memory index.
    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> Index<T, Self>;

    /// Converts a linear memory index to a 2-dimensional position.
    fn to_2d<T: Int>(index: Index<T, Self>, width: usize) -> Pos<T>;
}

/// Each row is stored contiguously in memory, with the first row at the lowest address.
pub struct RowMajor;

impl crate::internal::Sealed for RowMajor {}
impl Layout for RowMajor {
    const IS_ROW_MAJOR: bool = true;

    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> Index<T, Self> {
        Index::new(pos.y.to_usize() * width + pos.x.to_usize())
    }

    fn to_2d<T: Int>(index: Index<T, Self>, width: usize) -> Pos<T> {
        Pos {
            x: T::from_usize(index.index % width),
            y: T::from_usize(index.index / width),
        }
    }
}

/// Each column is stored contiguously in memory, with the first column at the lowest address.
pub struct ColMajor;

impl crate::internal::Sealed for ColMajor {}
impl Layout for ColMajor {
    const IS_ROW_MAJOR: bool = false;

    fn to_1d<T: Int>(pos: Pos<T>, width: usize) -> Index<T, Self> {
        Index {
            index: (pos.x.to_usize() * width + pos.y.to_usize()),
            _pos_type: PhantomData,
            _layout: PhantomData,
        }
    }

    fn to_2d<T: Int>(index: Index<T, Self>, width: usize) -> Pos<T> {
        Pos {
            x: T::from_usize(index.index / width),
            y: T::from_usize(index.index % width),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_row_major() {
        let pos = Pos::new(2, 3);
        let width = 5;

        let index = RowMajor::to_1d(pos, width);
        assert_eq!(index.index, 17); // 3 * 5 + 2
    }

    #[test]
    fn to_col_major() {
        let pos = Pos::new(2, 3);
        let width = 5;

        let index = ColMajor::to_1d(pos, width);
        assert_eq!(index.index, 13); // 2 * 5 + 3
    }

    #[test]
    fn from_row_major() {
        let index = Index::<i32, RowMajor>::new(17);
        let width = 5;

        let pos = RowMajor::to_2d(index, width);
        assert_eq!(pos.x, 2); // 17 % 5
        assert_eq!(pos.y, 3); // 17 / 5
    }

    #[test]
    fn from_col_major() {
        let index = Index::<i32, ColMajor>::new(13);
        let width = 5;

        let pos = ColMajor::to_2d(index, width);
        assert_eq!(pos.x, 2); // 13 / 5
        assert_eq!(pos.y, 3); // 13 % 5
    }
}
