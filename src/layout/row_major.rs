use core::{iter::FusedIterator, ops::Range};

use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{Linear, Traversal},
};

/// Left-to-right, top-to-bottom traversal order for 2D layouts.
///
/// ```txt
/// 0 1 2 3
/// 4 5 6 7
/// 8 9 A B
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RowMajor;

/// Iterator over positions in row-major order.
struct IterPosRowMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
}

impl<T: Int> Iterator for IterPosRowMajor<T> {
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterPosRowMajor<T> {
    fn len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        remaining_x.to_usize() * remaining_y.to_usize()
    }
}

impl<T: Int> FusedIterator for IterPosRowMajor<T> {}

/// Iterator over blocks in row-major order.
struct IterBlockRowMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
    size: Size,
}

impl<T: Int> Iterator for IterBlockRowMajor<T> {
    type Item = Rect<T>;

    fn next(&mut self) -> Option<Self::Item> {
        let block = Rect::new(self.current, self.size);
        self.current.x += T::from_usize(self.size.width);

        if self.current.x >= self.bounds.right() {
            self.current.x = self.bounds.left();
            self.current.y += T::from_usize(self.size.height);
        }

        if block.bottom() > self.bounds.bottom() || block.right() > self.bounds.right() {
            return None;
        }

        Some(block)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterBlockRowMajor<T> {
    fn len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        (remaining_x.to_usize() / self.size.width)
            .to_usize()
            .saturating_mul(remaining_y.to_usize() / self.size.height)
            .to_usize()
    }
}

impl<T: Int> FusedIterator for IterBlockRowMajor<T> {}

impl Traversal for RowMajor {
    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in row-major order.
    ///
    /// ## Examples
    ///
    /// ```txt
    /// (0, 0) (1, 0) (2, 0)
    /// (0, 1) (1, 1) (2, 1)
    /// ```
    ///
    /// ```rust
    /// use ixy::{Pos, Rect, layout::{Traversal, RowMajor}};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 3, 2);
    /// let traversal = RowMajor;
    /// let positions: Vec<_> = traversal.pos_iter(rect).collect();
    /// assert_eq!(
    ///     positions,
    ///     &[
    ///         Pos::new(0, 0),
    ///         Pos::new(1, 0),
    ///         Pos::new(2, 0),
    ///         Pos::new(0, 1),
    ///         Pos::new(1, 1),   
    ///         Pos::new(2, 1),
    ///     ]
    /// );
    /// ```
    fn iter_pos<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        let current = rect.top_left();
        IterPosRowMajor {
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
    /// [0, 0] [2, 0]
    /// [0, 2] [2, 2]
    /// ```
    ///
    /// ```rust
    /// use ixy::{Rect, Size, layout::{RowMajor, Traversal}};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// let traversal = RowMajor;
    /// let size = Size::new(2, 2);
    /// let blocks: Vec<_> = traversal.rect_iter(rect, size).collect();
    /// assert_eq!(
    ///     blocks,
    ///     &[
    ///         Rect::from_ltwh(0, 0, 2, 2),
    ///         Rect::from_ltwh(2, 0, 2, 2),
    ///         Rect::from_ltwh(0, 2, 2, 2),
    ///         Rect::from_ltwh(2, 2, 2, 2),
    ///     ]
    /// );
    /// ```
    fn iter_block<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>> {
        let current = rect.top_left();
        IterBlockRowMajor {
            current,
            bounds: rect,
            size,
        }
    }
}

impl RowMajor {
    const fn axis_to_range<E>(slice: &[E], size: Size, axis: usize) -> Range<usize> {
        assert!(
            slice.len() % size.area() == 0,
            "slice length must be a multiple of size.width * size.height"
        );
        let start = axis * size.width;
        let end = start + size.width;
        start..end
    }
}

impl Linear for RowMajor {
    fn pos_to_index(&self, pos: Pos<usize>, width: usize) -> usize {
        pos.y * width + pos.x
    }

    fn index_to_pos(&self, index: usize, width: usize) -> Pos<usize> {
        let x = index % width;
        let y = index / width;
        Pos::new(x, y)
    }

    fn len_aligned(&self, size: Size) -> usize {
        size.height
    }

    fn slice_aligned<'a, E>(&self, slice: &'a [E], size: Size, axis: usize) -> &'a [E] {
        if axis >= self.len_aligned(size) {
            return &[];
        }
        let range = Self::axis_to_range(slice, size, axis);
        &slice[range]
    }

    fn slice_aligned_mut<'a, E>(&self, slice: &'a mut [E], size: Size, axis: usize) -> &'a mut [E] {
        if axis >= self.len_aligned(size) {
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
    fn row_major_positions() {
        let rect = Rect::from_ltwh(0, 0, 2, 2);
        let traversal = RowMajor;
        let positions: Vec<_> = traversal.iter_pos(rect).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(0, 1),
                Pos::new(1, 1),
            ]
        );
    }

    #[test]
    fn row_major_blocks_full() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let traversal = RowMajor;
        let size = Size::new(2, 2);
        let blocks: Vec<_> = traversal.iter_block(rect, size).collect();
        assert_eq!(
            blocks,
            &[
                Rect::from_ltwh(0, 0, 2, 2),
                Rect::from_ltwh(2, 0, 2, 2),
                Rect::from_ltwh(0, 2, 2, 2),
                Rect::from_ltwh(2, 2, 2, 2),
            ]
        );
    }

    #[test]
    fn row_major_blocks_partial() {
        let rect = Rect::from_ltwh(0, 0, 5, 3);
        let traversal = RowMajor;
        let size = Size::new(2, 2);
        let blocks: Vec<_> = traversal.iter_block(rect, size).collect();
        assert_eq!(
            blocks,
            &[Rect::from_ltwh(0, 0, 2, 2), Rect::from_ltwh(2, 0, 2, 2),]
        );
    }

    #[test]
    fn row_major_to_1d() {
        assert_eq!(RowMajor.pos_to_index(Pos::new(0, 0), 2), 0);
        assert_eq!(RowMajor.pos_to_index(Pos::new(1, 0), 2), 1);
        assert_eq!(RowMajor.pos_to_index(Pos::new(0, 1), 2), 2);
        assert_eq!(RowMajor.pos_to_index(Pos::new(1, 1), 2), 3);
    }

    #[test]
    fn row_major_to_2d() {
        assert_eq!(RowMajor.index_to_pos(0, 2), Pos::new(0, 0));
        assert_eq!(RowMajor.index_to_pos(1, 2), Pos::new(1, 0));
        assert_eq!(RowMajor.index_to_pos(2, 2), Pos::new(0, 1));
        assert_eq!(RowMajor.index_to_pos(3, 2), Pos::new(1, 1));
    }

    #[test]
    fn slice_aligned_in_bounds() {
        #[rustfmt::skip]
        let slice = [
            0, 1, 2, 3, 
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(RowMajor.slice_aligned(&slice, size, 0), &[0, 1, 2, 3]);
        assert_eq!(RowMajor.slice_aligned(&slice, size, 1), &[4, 5, 6, 7]);
    }

    #[test]
    fn slice_aligned_out_of_bounds() {
        #[rustfmt::skip]
        let slice = [
            0, 1, 2, 3, 
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(RowMajor.slice_aligned(&slice, size, 2), &[]);
    }
}
