//! Maps 2-dimensional positions and provides traversal orders.
//!
//! Defines the [`Layout`] trait for iterating over positions and rectangles in a 2D layout,
//! with 3 built-in implementations:
//!
//! - [`RowMajor`] for row-major order
//! - [`ColumnMajor`] for column-major order
//! - [`Block`] for block-based traversal (where the inner blocks can themselves have a layout)
//!
//! In addition, the [`LinearLayout`] trait provides mapping and iterating methods for linear data.

use core::ops::Range;

use crate::{Pos, Rect, Size, int::Int};

mod block;
pub use block::Block;

mod col_major;
pub use col_major::ColumnMajor;

mod row_major;
pub use row_major::RowMajor;

/// Defines iterating orders for traversing a 2D layout.
pub trait Layout {
    /// Returns an iterator over the positions.
    ///
    /// The positions are returned in the order defined by the traversal.
    ///
    /// Positions that would be partially outside the rectangle are not yielded.
    fn iter_pos<T: Int>(rect: Rect<T>) -> impl Iterator<Item = Pos<T>>;

    /// Returns an iterator over blocks (smaller, equally-sized rectangles).
    ///
    /// The blocks are returned in the order defined by the traversal.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    fn iter_rect<T: Int>(rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>>;
}

/// Defines mapping a 2D layout to a linear access patterns.
pub trait LinearLayout: Layout {
    /// Translates a 2D position to a linear index for the current layout.
    ///
    /// `stride` is the number of elements between the start of one "line" and the next in the
    /// layout's primary iteration axis: the row width for [`RowMajor`], or the column height for
    /// [`ColumnMajor`].
    ///
    /// The coordinate type `T` can be any [`Int`] (for example `u16`, the natural type for
    /// terminal or pixel grids); the resulting index is always a [`usize`], since it is a buffer
    /// offset rather than a 2D coordinate.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, layout::{LinearLayout, RowMajor}};
    ///
    /// // `u16` coordinates, as commonly used for terminal grids.
    /// assert_eq!(RowMajor::pos_to_index(Pos::<u16>::new(1, 1), 2), 3);
    /// ```
    #[must_use]
    fn pos_to_index<T: Int>(pos: Pos<T>, stride: usize) -> usize;

    /// Translates a linear index to a 2D position for the current layout.
    ///
    /// See [`LinearLayout::pos_to_index`] for the meaning of `stride`.
    ///
    /// The returned coordinates are converted from `usize` using [`Int::from_usize`], which
    /// follows the crate's standard conversion policy: it panics in debug builds if the value
    /// does not fit in `T`, and saturates to `T::MAX` in release builds. If you need a
    /// non-panicking, non-lossy conversion, use [`LinearLayout::checked_index_to_pos`] instead.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, layout::{LinearLayout, RowMajor}};
    ///
    /// // `u16` coordinates, as commonly used for terminal grids.
    /// assert_eq!(RowMajor::index_to_pos::<u16>(3, 2), Pos::new(1, 1));
    /// ```
    #[must_use]
    fn index_to_pos<T: Int>(index: usize, stride: usize) -> Pos<T>;

    /// Like [`LinearLayout::index_to_pos`], but returns `None` if the resulting coordinates do
    /// not fit in `T`, instead of panicking (in debug builds) or saturating (in release builds).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::layout::{LinearLayout, RowMajor};
    ///
    /// // Fits in a `u8`.
    /// assert!(RowMajor::checked_index_to_pos::<u8>(3, 2).is_some());
    ///
    /// // Does not fit in a `u8`, since the resulting `x` coordinate is `300`.
    /// assert!(RowMajor::checked_index_to_pos::<u8>(300, 400).is_none());
    /// ```
    #[must_use]
    fn checked_index_to_pos<T: Int>(index: usize, stride: usize) -> Option<Pos<T>> {
        let pos = Self::index_to_pos::<usize>(index, stride);
        Some(Pos::new(
            T::checked_from_usize(pos.x)?,
            T::checked_from_usize(pos.y)?,
        ))
    }

    /// Returns the length of the linear data for the given size and axis.
    ///
    /// This is the maximum value that can be provided as `axis` to `slice_aligned`.
    #[must_use]
    fn len_aligned(size: Size) -> usize;

    /// Returns a range of indices for the rectangle defined by the layout.
    ///
    /// The range is inclusive of the start and exclusive of the end.
    ///
    /// If the rectangle is not aligned to the current data, the range will be `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Size, layout::{LinearLayout, RowMajor}};
    ///
    /// // `u16` coordinates, as commonly used for terminal grids.
    /// let size = Size::<u16>::new(4, 2);
    /// let rect = Rect::<u16>::from_ltwh(0, 1, 4, 1);
    /// assert_eq!(RowMajor::rect_to_range(size, rect), Some(4..8));
    /// ```
    #[must_use]
    fn rect_to_range<T: Int>(size: Size<T>, rect: Rect<T>) -> Option<Range<usize>>;

    /// Returns a slice of the given slice for the rectangle defined by the layout.
    ///
    /// If the rectangle is not aligned to the current data, the slice will be `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Size, layout::{LinearLayout, RowMajor}};
    ///
    /// // `u16` coordinates, as commonly used for terminal grids.
    /// let slice = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let size = Size::<u16>::new(4, 2);
    /// let rect = Rect::<u16>::from_ltwh(0, 1, 4, 1);
    /// assert_eq!(RowMajor::slice_rect_aligned(&slice, size, rect), Some(&[4, 5, 6, 7][..]));
    /// ```
    #[must_use]
    fn slice_rect_aligned<T: Int, E>(slice: &[E], size: Size<T>, rect: Rect<T>) -> Option<&[E]>;

    /// Returns a mutable slice of the given slice for the rectangle defined by the layout.
    ///
    /// If the rectangle is not aligned to the current data, the slice will be `None`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Size, layout::{LinearLayout, RowMajor}};
    ///
    /// // `u16` coordinates, as commonly used for terminal grids.
    /// let mut slice = [0, 1, 2, 3, 4, 5, 6, 7];
    /// let size = Size::<u16>::new(4, 2);
    /// let rect = Rect::<u16>::from_ltwh(0, 1, 4, 1);
    /// assert_eq!(
    ///     RowMajor::slice_rect_aligned_mut(&mut slice, size, rect),
    ///     Some(&mut [4, 5, 6, 7][..])
    /// );
    /// ```
    #[must_use]
    fn slice_rect_aligned_mut<T: Int, E>(
        slice: &mut [E],
        size: Size<T>,
        rect: Rect<T>,
    ) -> Option<&mut [E]>;

    /// Returns a slice of the given slice for the axis defined by the layout.
    ///
    /// If the axis is not present in the data, the slice will be empty.
    ///
    /// ## Panics
    ///
    /// If `slice.len()` is not a multiple of `size.width * size.height`, this method will panic.
    #[must_use]
    fn slice_aligned<E>(slice: &[E], size: Size, axis: usize) -> &[E];

    /// Returns a mutable slice of the given slice for the axis defined by the layout.
    ///
    /// If the axis is not present in the data, the slice will be empty.
    ///
    /// ## Panics
    ///
    /// If `slice.len()` is not a multiple of `size.width * size.height`, this method will panic.
    #[must_use]
    fn slice_aligned_mut<E>(slice: &mut [E], size: Size, axis: usize) -> &mut [E];
}
