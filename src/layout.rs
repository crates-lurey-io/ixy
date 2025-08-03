//! Maps 2-dimensional positions and provides traversal orders.

use crate::{Pos, Rect, Size, int::Int};

mod block;
pub use block::Block;

mod col_major;
pub use col_major::ColumnMajor;

mod row_major;
pub use row_major::RowMajor;

/// Defines iterating orders for traversing a 2D layout.
pub trait Traversal {
    /// Type of the iterator for positions.
    type PosIter<'a, T: Int>: Iterator<Item = Pos<T>>
    where
        Self: 'a;

    /// Type of the iterator for blocks.
    type BlockIter<'a, T: Int>: Iterator<Item = Rect<T>>
    where
        Self: 'a;

    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in the order defined by the traversal.
    fn positions<T: Int>(&self, rect: Rect<T>) -> Self::PosIter<'_, T>;

    /// Returns an iterator over blocks of the specified size within the rectangle.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn blocks<T: Int>(&self, rect: Rect<T>, size: Size) -> Self::BlockIter<'_, T>;
}

/// Defines the mapping of 2D positions to a 1D index and vice versa.
pub trait LinearLayout {
    /// Given a 2-dimensional position and a width, returns the corresponding 1D index.
    fn to_1d<T: Int>(&self, pos: Pos<T>, width: usize) -> usize;

    /// Given a 1D index and a width, returns the corresponding 2-dimensional position.
    fn to_2d<T: Int>(&self, index: usize, width: usize) -> Pos<T>;
}
