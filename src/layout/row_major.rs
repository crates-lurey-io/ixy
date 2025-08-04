use core::iter::FusedIterator;

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
    fn pos_iter<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>> {
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
    fn rect_iter<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>> {
        let current = rect.top_left();
        IterBlockRowMajor {
            current,
            bounds: rect,
            size,
        }
    }
}

impl Linear for RowMajor {
    fn to_1d<T: Int>(&self, pos: Pos<T>, width: usize) -> usize {
        pos.y.to_usize() * width + pos.x.to_usize()
    }

    fn to_2d<T: Int>(&self, index: usize, width: usize) -> Pos<T> {
        let x = index % width;
        let y = index / width;
        Pos::new(T::from_usize(x), T::from_usize(y))
    }

    unsafe fn iter_rect_unchecked<'a, T: Int, E>(
        &'a self,
        rect: Rect<usize>,
        size: Size,
        data: &'a [E],
    ) -> impl Iterator<Item = &'a E> {
        debug_assert!(
            data.len() == size.width * size.height,
            "Data length does not match the area of the size"
        );
        debug_assert!(
            rect.left() + rect.width() <= size.width && rect.top() + rect.height() <= size.height,
            "Rectangle {rect:?} is out of bounds for size {size:?}"
        );
        (0..rect.height()).flat_map(move |row| {
            let start = (rect.top() + row) * size.width + rect.left();
            let slice = unsafe {
                let ptr = data.as_ptr().add(start);
                core::slice::from_raw_parts(ptr, rect.width())
            };
            slice.iter()
        })
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::layout::Block;

    use super::*;
    use alloc::{vec, vec::Vec};

    #[test]
    fn row_major_positions() {
        let rect = Rect::from_ltwh(0, 0, 2, 2);
        let traversal = RowMajor;
        let positions: Vec<_> = traversal.pos_iter(rect).collect();
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
        let blocks: Vec<_> = traversal.rect_iter(rect, size).collect();
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
        let blocks: Vec<_> = traversal.rect_iter(rect, size).collect();
        assert_eq!(
            blocks,
            &[Rect::from_ltwh(0, 0, 2, 2), Rect::from_ltwh(2, 0, 2, 2),]
        );
    }

    #[test]
    fn row_major_iter_rect_from_slice() {
        let traversal = RowMajor;
        let rect = Rect::from_ltwh(0, 0, 3, 2);
        let size = Size::new(3, 2);
        let data: Vec<i32> = (0..6).collect(); // 3x2 grid
        let iter: Vec<_> = traversal.iter_rect(rect, size, &data).collect();
        assert_eq!(iter, &[&0, &1, &2, &3, &4, &5]);
    }

    #[test]
    fn col_major_iter_rect_from_slice() {
        let traversal = RowMajor;
        let rect = Rect::from_ltwh(0, 0, 3, 2);
        let size = Size::new(3, 2);
        let data: Vec<i32> = (0..6).collect(); // 3x2 grid
        let iter: Vec<_> = traversal.iter_rect(rect, size, &data).collect();
        assert_eq!(iter, &[&0, &1, &2, &3, &4, &5]);
    }

    #[test]
    fn block_row_major_row_major_from_slice() {
        let traversal = Block::row_major(2, 2);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let size = Size::new(4, 4);
        let data: Vec<i32> = (0..16).collect(); // 4x4 grid
        let iter: Vec<_> = traversal.iter_rect(rect, size, &data).collect();
        assert_eq!(
            iter,
            &[
                &0, &1, &2, &3, &4, &5, &6, &7, &8, &9, &10, &11, &12, &13, &14, &15
            ]
        );
    }

    #[test]
    fn row_major_to_1d() {
        assert_eq!(RowMajor.to_1d(Pos::new(0, 0), 2), 0);
        assert_eq!(RowMajor.to_1d(Pos::new(1, 0), 2), 1);
        assert_eq!(RowMajor.to_1d(Pos::new(0, 1), 2), 2);
        assert_eq!(RowMajor.to_1d(Pos::new(1, 1), 2), 3);
    }

    #[test]
    fn row_major_to_2d() {
        assert_eq!(RowMajor.to_2d(0, 2), Pos::new(0, 0));
        assert_eq!(RowMajor.to_2d(1, 2), Pos::new(1, 0));
        assert_eq!(RowMajor.to_2d(2, 2), Pos::new(0, 1));
        assert_eq!(RowMajor.to_2d(3, 2), Pos::new(1, 1));
    }

    #[test]
    #[should_panic(expected = "Data length does not match the area of the size")]
    fn row_major_iter_rect_unchecked_panic_data_length_mismatch() {
        let traversal = RowMajor;
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let size = Size::new(2, 2);
        let data: Vec<i32> = (0..5).collect(); // 2x2 grid, but only 5 elements
        let _iter: Vec<_> = traversal.iter_rect(rect, size, &data).collect();
    }

    #[test]
    #[should_panic(
        expected = "Data length does not match the area of the size\n  left: 2\n right: 9"
    )]
    fn row_major_iter_rect_unchecked_panic_out_of_bounds() {
        let traversal = RowMajor;
        let rect = Rect::from_ltwh(0, 0, 5, 5);
        let size = Size::new(3, 3);
        let data = vec![0, 9];
        let _iter: Vec<_> = traversal.iter_rect(rect, size, &data).collect();
    }
}
