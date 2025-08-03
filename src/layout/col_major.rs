use core::iter::FusedIterator;

use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{LinearLayout, Traversal},
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
pub struct IterPosColMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
}

impl<T: Int> IterPosColMajor<T> {
    fn remaining_len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        remaining_x.to_usize() * remaining_y.to_usize()
    }
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
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterPosColMajor<T> {
    fn len(&self) -> usize {
        self.remaining_len()
    }
}

impl<T: Int> FusedIterator for IterPosColMajor<T> {}

/// Iterator over blocks in column-major order.
pub struct IterBlockColMajor<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
    size: Size,
}

impl<T: Int> IterBlockColMajor<T> {
    fn remaining_len(&self) -> usize {
        let remaining_x = self.bounds.right() - self.current.x;
        let remaining_y = self.bounds.bottom() - self.current.y;
        (remaining_x.to_usize() / self.size.width) * (remaining_y.to_usize() / self.size.height)
    }
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
        let len = self.remaining_len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IterBlockColMajor<T> {
    fn len(&self) -> usize {
        self.remaining_len()
    }
}

impl<T: Int> FusedIterator for IterBlockColMajor<T> {}

impl Traversal for ColumnMajor {
    type PosIter<'a, T: Int> = IterPosColMajor<T>;
    type BlockIter<'a, T: Int> = IterBlockColMajor<T>;

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
    /// let positions: Vec<_> = traversal.positions(&rect).collect();
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
    fn positions<T: Int>(&self, rect: Rect<T>) -> Self::PosIter<'_, T> {
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
    /// let blocks: Vec<_> = traversal.blocks(&rect, size).collect();
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
    fn blocks<T: Int>(&self, rect: Rect<T>, size: Size) -> Self::BlockIter<'_, T> {
        let current = rect.top_left();
        IterBlockColMajor {
            current,
            bounds: rect,
            size,
        }
    }
}

impl LinearLayout for ColumnMajor {
    fn to_1d<T: Int>(&self, pos: Pos<T>, width: usize) -> usize {
        pos.x.to_usize() * width + pos.y.to_usize()
    }

    fn to_2d<T: Int>(&self, index: usize, width: usize) -> Pos<T> {
        let x = index % width;
        let y = index / width;
        Pos::new(T::from_usize(x), T::from_usize(y))
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
        let positions: Vec<_> = traversal.positions(rect).collect();
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
    fn column_major_blocks_full() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let traversal = ColumnMajor;
        let size = Size::new(2, 2);
        let blocks: Vec<_> = traversal.blocks(rect, size).collect();
        assert_eq!(
            blocks,
            &[
                Rect::from_ltwh(0, 0, 2, 2),
                Rect::from_ltwh(0, 2, 2, 2),
                Rect::from_ltwh(2, 0, 2, 2),
                Rect::from_ltwh(2, 2, 2, 2),
            ]
        );
    }

    #[test]
    fn column_major_blocks_partial() {
        let rect = Rect::from_ltwh(0, 0, 3, 5);
        let traversal = ColumnMajor;
        let size = Size::new(2, 2);
        let blocks: Vec<_> = traversal.blocks(rect, size).collect();
        assert_eq!(
            blocks,
            &[Rect::from_ltwh(0, 0, 2, 2), Rect::from_ltwh(0, 2, 2, 2),]
        );
    }

    #[test]
    fn column_major_to_1d() {
        let traversal = ColumnMajor;
        let pos = Pos::new(1, 2);
        assert_eq!(traversal.to_1d(pos, 4), 5);
    }

    #[test]
    fn column_major_to_2d() {
        let traversal = ColumnMajor;
        let index = 5;
        let pos = traversal.to_2d(index, 4);
        assert_eq!(pos, Pos::new(1, 2));
    }
}
