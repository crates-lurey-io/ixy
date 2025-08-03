//! Maps 2-dimensional positions and provides traversal orders.

use crate::{HasSize as _, Pos, Rect, Size, int::Int};

mod block;
pub use block::Block;

mod col_major;
pub use col_major::ColumnMajor;

mod row_major;
pub use row_major::RowMajor;

/// Defines iterating orders for traversing a 2D layout.
pub trait Layout {
    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in the order defined by the traversal.
    fn pos_iter<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>>;

    /// Returns an iterator over blocks of the specified size within the rectangle.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn rect_iter<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>>;
}

/// Defines iterating orders for traversing a 2D layout with linear access patterns.
pub trait Linear: Layout {
    /// Given a 2-dimensional position and a width, returns the corresponding 1D index.
    fn to_1d<T: Int>(&self, pos: Pos<T>, width: usize) -> usize;

    /// Given a 1D index and a width, returns the corresponding 2-dimensional position.
    fn to_2d<T: Int>(&self, index: usize, width: usize) -> Pos<T>;

    /// Returns an iterator of elements in a rectangle, assuming `data` represents this layout.
    ///
    /// The elements are returned in the order defined by the traversal.
    ///
    /// If the rectangle is larger than the data, it will yield only the elements that fit within
    /// the rectangle.
    ///
    /// ## Panics
    ///
    /// If the length of `data` is not equal to the area of `size`.
    fn iter_rect<'a, T: Int, E>(
        &'a self,
        rect: Rect<T>,
        size: Size,
        data: &'a [E],
    ) -> impl Iterator<Item = &'a E> {
        assert_eq!(
            data.len(),
            size.width * size.height,
            "Data length does not match the area of the size"
        );
        let rect = Rect::<usize>::from_ltwh(
            T::saturating_to_usize(rect.left()),
            T::saturating_to_usize(rect.top()),
            rect.width().min(size.width),
            rect.height().min(size.height),
        );
        let rect = rect.intersect(size.to_rect());
        unsafe { self.iter_rect_unchecked::<usize, E>(rect, size, data) }
    }

    /// Returns an iterator of elements in a rectangle, assuming `data` represents this layout.
    ///
    /// The elements are returned in the order defined by the traversal.
    ///
    /// ## Safety
    ///
    /// This method assumes that:
    ///
    /// - The `data` slice is large enough to cover the rectangle defined by `rect` and `size`.
    /// - The `rect` is within the bounds of the data.
    ///
    /// If either of these conditions are not met, it may lead to undefined behavior.
    unsafe fn iter_rect_unchecked<'a, T: Int, E>(
        &'a self,
        rect: Rect<usize>,
        size: Size,
        data: &'a [E],
    ) -> impl Iterator<Item = &'a E>;
}
