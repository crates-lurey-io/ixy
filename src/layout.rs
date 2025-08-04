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

use core::ops::Range;

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
    fn iter_rect<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>>;
}

/// Defines mapping a 2D layout to a linear access patterns.
pub trait Linear: Traversal {
    /// Translates a 2D position to a linear index for the current layout.
    #[must_use]
    fn pos_to_index(&self, pos: Pos<usize>, width: usize) -> usize;

    /// Translates a linear index to a 2D position for the current layout.
    #[must_use]
    fn index_to_pos(&self, index: usize, width: usize) -> Pos<usize>;

    /// Returns the length of the linear data for the given size and axis.
    ///
    /// This is the maximum value that can be provided as `axis` to `slice_aligned`.
    #[must_use]
    fn len_aligned(&self, size: Size) -> usize;

    /// Returns a range of indices for the rectangle defined by the layout.
    ///
    /// The range is inclusive of the start and exclusive of the end.
    ///
    /// If the rectangle is not aligned to the current data, the range will be `None`.
    #[must_use]
    fn rect_to_range(&self, size: Size, rect: Rect<usize>) -> Option<Range<usize>>;

    /// Returns a slice of the given slice for the rectangle defined by the layout.
    ///
    /// If the rectangle is not aligned to the current data, the slice will be `None`.
    fn slice_rect_aligned<'a, E>(
        &self,
        slice: &'a [E],
        size: Size,
        rect: Rect<usize>,
    ) -> Option<&'a [E]>;

    /// Returns a mutable slice of the given slice for the rectangle defined by the layout.
    ///
    /// If the rectangle is not aligned to the current data, the slice will be `None`.
    fn slice_rect_aligned_mut<'a, E>(
        &self,
        slice: &'a mut [E],
        size: Size,
        rect: Rect<usize>,
    ) -> Option<&'a mut [E]>;

    /// Returns a slice of the given slice for the axis defined by the layout.
    ///
    /// If the axis is not present in the data, the slice will be empty.
    ///
    /// ## Panics
    ///
    /// If `slice.len()` is not a multiple of `size.width * size.height`, this method will panic.
    #[must_use]
    fn slice_aligned<'a, E>(&self, slice: &'a [E], size: Size, axis: usize) -> &'a [E];

    /// Returns a mutable slice of the given slice for the axis defined by the layout.
    ///
    /// If the axis is not present in the data, the slice will be empty.
    ///
    /// ## Panics
    ///
    /// If `slice.len()` is not a multiple of `size.width * size.height`, this method will panic.
    #[must_use]
    fn slice_aligned_mut<'a, E>(&self, slice: &'a mut [E], size: Size, axis: usize) -> &'a mut [E];
}
