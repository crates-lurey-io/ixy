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
    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in the order defined by the traversal.
    fn positions<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>>;

    /// Returns an iterator over blocks of the specified size within the rectangle.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn blocks<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>>;
}

/// Defines the mapping of 2D positions to a 1D index and vice versa.
pub trait LinearLayout {
    /// Given a 2-dimensional position and a width, returns the corresponding 1D index.
    fn to_1d<T: Int>(&self, pos: Pos<T>, width: usize) -> usize;

    /// Given a 1D index and a width, returns the corresponding 2-dimensional position.
    fn to_2d<T: Int>(&self, index: usize, width: usize) -> Pos<T>;
}
