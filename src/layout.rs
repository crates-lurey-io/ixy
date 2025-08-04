//! Maps 2-dimensional positions and provides traversal orders.
//!
//! Defines the [`Traversal`] trait for iterating over positions and rectangles in a 2D layout,
//! with 3 built-in implementations:
//!
//! - [`RowMajor`] for row-major order
//! - [`ColumnMajor`] for column-major order
//! - [`Block`] for block-based traversal (where the inner blocks can themselves have a layout)
//!
//! In addition, the [`Linear`] trait provides mapping and iterating methods for linear data.

use crate::{Pos, Rect, Size, int::Int};

mod block;
pub use block::Block;

mod col_major;
pub use col_major::ColumnMajor;

mod row_major;
pub use row_major::RowMajor;

/// Defines iterating orders for traversing a 2D layout.
pub trait Traversal {
    /// Returns an iterator over the positions.
    ///
    /// The positions are returned in the order defined by the traversal.
    ///
    /// Positions that would be partially outside the rectangle are not yielded.
    fn iter_pos<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>>;

    /// Returns an iterator over blocks (smaller, equally-sized rectangles).
    ///
    /// The blocks are returned in the order defined by the traversal.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn iter_block<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>>;
}

/// Defines mapping a 2D layout to a linear access patterns.
pub trait Linear {
    /// Translates a 2D position to a linear index for the current layout.
    #[must_use]
    fn pos_to_index(&self, pos: Pos<usize>, width: usize) -> usize;

    /// Translates a linear index to a 2D position for the current layout.
    #[must_use]
    fn index_to_pos(&self, index: usize, width: usize) -> Pos<usize>;
}
