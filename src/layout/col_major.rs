use core::{iter::FusedIterator, ops::Range};

use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{Linear, Traversal},
};

/// Top-to-bottom, left-to-right traversal order for 2D layouts.
///
/// ```txt
/// 0 3 6 9
/// 1 4 7 A
/// 2 5 8 B
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ColumnMajor;

/// Iterator over positions in column-major order.
struct IterPosColMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
}
impl<T: Int> Iterator for IterPosColMajor<T> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterPosColMajor<T> {
    fn len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        remaining_x.to_usize() * remaining_y.to_usize()
    }
}

impl<T: Int> FusedIterator for IterPosColMajor<T> {}

/// Iterator over blocks in column-major order.
struct IterBlockColMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
    size: Size,
}

impl<T: Int> Iterator for IterBlockColMajor<T> {
    type Item = Rect<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.x >= self.bounds.right() {
            return None;
        }
        let block = Rect::new(self.current, self.size);
        self.current.y += T::from_usize(self.size.height);

        if self.current.y >= self.bounds.bottom() {
            self.current.y = self.bounds.top();
            self.current.x += T::from_usize(self.size.width);
        }

        if block.bottom() > self.bounds.bottom() || block.right() > self.bounds.right() {
            return None; // Block is partially outside the rectangle
        }

        Some(block)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterBlockColMajor<T> {
    fn len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        (remaining_x.to_usize() / self.size.width) * (remaining_y.to_usize() / self.size.height)
    }
}

impl<T: Int> FusedIterator for IterBlockColMajor<T> {}

impl Traversal for ColumnMajor {
    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in column-major order.
    ///
    /// ## Examples
    ///
    /// ```txt
    /// (0, 0) (0, 1)
    /// (1, 0) (1, 1)
    /// (2, 0) (2, 1)
    /// ```
    /// ```rust
    /// use ixy::{Pos, Rect, layout::{ColumnMajor, Traversal}};
    /// let rect = Rect::from_ltwh(0, 0, 3, 2);
    /// let traversal = ColumnMajor;
    /// let positions: Vec<_> = traversal.pos_iter(rect).collect();
    /// assert_eq!(
    ///     positions,
    ///     &[
    ///         Pos::new(0, 0),
    ///         Pos::new(0, 1),
    ///         Pos::new(1, 0),
    ///         Pos::new(1, 1),
    ///         Pos::new(2, 0),
    ///         Pos::new(2, 1),
    ///     ]
    /// );
    /// ```
    fn iter_pos<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        let current = rect.top_left();
        IterPosColMajor {
            current,
            bounds: rect,
        }
    }

    /// Returns an iterator over blocks of the specified size within the rectangle.
    ///
    /// Blocks that would be partially outside the rectangle are not yielded.
    ///
    /// ## Examples
    ///
    /// ```txt
    /// [0, 0] [0, 2]
    /// [2, 0] [2, 2]
    /// ```
    /// ```rust
    /// use ixy::{Rect, Size, layout::{ColumnMajor, Traversal}};
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// let traversal = ColumnMajor;
    /// let size = Size::new(2, 2);
    /// let blocks: Vec<_> = traversal.rect_iter(rect, size).collect();
    /// assert_eq!(
    ///     blocks,
    ///     &[
    ///         Rect::from_ltwh(0, 0, 2, 2),
    ///         Rect::from_ltwh(0, 2, 2, 2),
    ///         Rect::from_ltwh(2, 0, 2, 2),
    ///         Rect::from_ltwh(2, 2, 2, 2),
    ///     ]
    /// );
    /// ```
    fn iter_block<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>> {
        let current = rect.top_left();
        IterBlockColMajor {
            current,
            bounds: rect,
            size,
        }
    }
}

impl ColumnMajor {
    const fn axis_to_range<E>(slice: &[E], size: Size, axis: usize) -> Range<usize> {
        assert!(
            slice.len() % size.area() == 0,
            "slice length must be a multiple of size.width * size.height"
        );
        let start = axis * size.height;
        let end = start + size.height;
        start..end
    }
}

impl Linear for ColumnMajor {
    fn pos_to_index(&self, pos: Pos<usize>, width: usize) -> usize {
        pos.x * width + pos.y
    }

    fn index_to_pos(&self, index: usize, width: usize) -> Pos<usize> {
        let x = index / width;
        let y = index % width;
        Pos::new(x, y)
    }

    fn len_aligned(&self, size: Size) -> usize {
        size.width
    }

    fn slice_aligned<'a, E>(&self, slice: &'a [E], size: Size, axis: usize) -> &'a [E] {
        if axis >= size.width {
            return &[];
        }
        let range = Self::axis_to_range(slice, size, axis);
        &slice[range]
    }

    fn slice_aligned_mut<'a, E>(&self, slice: &'a mut [E], size: Size, axis: usize) -> &'a mut [E] {
        if axis >= size.width {
            return &mut [];
        }
        let range = Self::axis_to_range(slice, size, axis);
        &mut slice[range]
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn column_major_positions() {
        let rect = Rect::from_ltwh(0, 0, 3, 2);
        let traversal = ColumnMajor;
        let positions: Vec<_> = traversal.iter_pos(rect).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(1, 0),
                Pos::new(1, 1),
                Pos::new(2, 0),
                Pos::new(2, 1),
            ]
        );
    }

    #[test]
    fn column_major_to_1d() {
        assert_eq!(ColumnMajor.pos_to_index(Pos::new(0, 0), 2), 0);
        assert_eq!(ColumnMajor.pos_to_index(Pos::new(0, 1), 2), 1);
        assert_eq!(ColumnMajor.pos_to_index(Pos::new(1, 0), 2), 2);
        assert_eq!(ColumnMajor.pos_to_index(Pos::new(1, 1), 2), 3);
    }

    #[test]
    fn column_major_to_2d() {
        assert_eq!(ColumnMajor.index_to_pos(0, 2), Pos::new(0, 0));
        assert_eq!(ColumnMajor.index_to_pos(1, 2), Pos::new(0, 1));
        assert_eq!(ColumnMajor.index_to_pos(2, 2), Pos::new(1, 0));
        assert_eq!(ColumnMajor.index_to_pos(3, 2), Pos::new(1, 1));
    }

    #[test]
    fn column_major_exact_size_iter_pos_len() {
        let rect = Rect::from_ltwh(0, 0, 3, 2);
        let traversal = ColumnMajor;
        let iter: Vec<_> = traversal.iter_pos(rect).collect();
        assert_eq!(iter.len(), 6);
    }

    #[test]
    fn slice_aligned_mut() {
        #[rustfmt::skip]
        let slice = &mut [
            1, 2, 3,
            4, 5, 6
        ];
        let size = Size::new(2, 3);
        assert_eq!(
            ColumnMajor.slice_aligned_mut(slice, size, 0),
            &mut [1, 2, 3]
        );
        assert_eq!(
            ColumnMajor.slice_aligned_mut(slice, size, 1),
            &mut [4, 5, 6]
        );
    }

    #[test]
    fn slice_aligned_in_bounds() {
        #[rustfmt::skip]
        let slice = &[
            1, 2, 3,
            4, 5, 6
        ];
        let size = Size::new(2, 3);
        assert_eq!(ColumnMajor.slice_aligned(slice, size, 0), &[1, 2, 3]);
        assert_eq!(ColumnMajor.slice_aligned(slice, size, 1), &[4, 5, 6]);
    }

    #[test]
    fn slice_aligned_out_of_bounds() {
        #[rustfmt::skip]
        let slice = &[
            1, 2, 3,
            4, 5, 6
        ];
        let size = Size::new(2, 3);
        assert_eq!(ColumnMajor.slice_aligned(slice, size, 2), &[]);
    }
}
