use core::iter::FusedIterator;

use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{ColumnMajor, RowMajor, Traversal},
};

/// 2D space divided into blocks, each containing a grid of cells.
///
/// Each block has a fixed size (that may be defined at runtime), and is traversed using layout `G`
/// for each block, and layout `C` for each cell within the block; by default, both are `RowMajor`
/// but can be customized using the [`with_grid`] and [`with_cell`] methods.
///
/// [`with_grid`]: Block::with_grid
/// [`with_cell`]: Block::with_cell
///
/// For example, `Block<RowMajor, RowMajor>` with a block-size of 2x2:
///
/// ```txt
/// B0:   B1:
/// +----+----+
/// | 01 | 45 |
/// | 23 | 67 |
/// +----+----+
/// B2:   B3:
/// +----+----+
/// | 89 | CD |
/// | AB | EF |
/// +----+----+
/// ```
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, Rect, Size, layout::{Block, Traversal}};
///
/// let rect = Rect::from_ltwh(0, 0, 4, 4);
/// let block = Block::row_major(2, 2);
/// let cells: Vec<_> = block.positions(rect).collect();
/// assert_eq!(
///    cells,
///    &[
///       // Block 0
///       Pos::new(0, 0),
///       Pos::new(1, 0),
///       Pos::new(0, 1),
///       Pos::new(1, 1),
///
///       // Block 1
///       Pos::new(2, 0),
///       Pos::new(3, 0),
///       Pos::new(2, 1),
///       Pos::new(3, 1),
///
///       // Block 2
///       Pos::new(0, 2),
///       Pos::new(1, 2),
///       Pos::new(0, 3),
///       Pos::new(1, 3),
///
///       // Block 3
///       Pos::new(2, 2),
///       Pos::new(3, 2),
///       Pos::new(2, 3),
///       Pos::new(3, 3),
///     ]
/// );
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block<G, C> {
    size: Size,
    grid: G,
    cell: C,
}

impl<G: Copy, C: Copy> Block<G, C> {
    /// Creates a new block layout with the specified width, height, and traversal orders.
    #[must_use]
    pub const fn new(width: usize, height: usize, grid: G, cell: C) -> Self {
        Block {
            size: Size { width, height },
            grid,
            cell,
        }
    }
}

impl Block<RowMajor, RowMajor> {
    /// Creates a new block layout with the specified width and height.
    ///
    /// Defaults to blocks being laid out in a row-major order, with cells in a row-major order.
    #[must_use]
    pub const fn row_major(width: usize, height: usize) -> Self {
        Block {
            size: Size { width, height },
            grid: RowMajor,
            cell: RowMajor,
        }
    }
}

impl Block<ColumnMajor, ColumnMajor> {
    /// Creates a new block layout with the specified width and height.
    ///
    /// Defaults to blocks being laid out in a column-major order, with cells in a column-major order.
    #[must_use]
    pub const fn col_major(width: usize, height: usize) -> Self {
        Block {
            size: Size { width, height },
            grid: ColumnMajor,
            cell: ColumnMajor,
        }
    }
}

impl<B: Copy, C: Copy> Block<B, C> {
    /// Transforms the block layout with the provided way to traverse the grid for blocks.
    ///
    /// This allows for different traversal orders for the blocks themselves.
    #[must_use]
    pub const fn with_grid<BT>(self, grid_layout: BT) -> Block<BT, C> {
        Block {
            grid: grid_layout,
            size: self.size,
            cell: self.cell,
        }
    }

    /// Transforms the block layout with the provided way to traverse the blocks for cells.
    ///
    /// This allows for different traversal orders for the cells within each block.
    #[must_use]
    pub const fn with_cell<CT>(self, cell_layout: CT) -> Block<B, CT> {
        Block {
            cell: cell_layout,
            size: self.size,
            grid: self.grid,
        }
    }
}

// /// Determines which block number the given index belongs to, given the block size.
// ///
// /// Returns both the block number and the offset within the block.
// fn get_block_by_index(index: usize, block_size: usize) -> (usize, usize) {
//     let block = index / block_size;
//     let offset = index % block_size;
//     (block, offset)
// }

// /// Determines which block number the given position belongs to, given the block size.
// ///
// /// Returns both the block number and the offset within the block.
// fn get_block_by_pos<T: Int>(pos: Pos<T>, size: usize) -> (usize, usize) {
//     let block_x = pos.x.to_usize() / size;
//     let block_y = pos.y.to_usize() / size;
//     let offset_x = pos.x.to_usize() % size;
//     let offset_y = pos.y.to_usize() % size;
//     let block = block_y + block_x;
//     let offset = offset_y * size + offset_x;
//     (block, offset)
// }

/// Iterator over positions defined by blocks `G` and interior cells `C`.
pub struct IterPosBlock<'a, T: Int, G, C>
where
    G: Traversal,
    C: Traversal,
{
    layout: &'a Block<G, C>,
    current_block: Rect<T>,
    block_iter: G::BlockIter<'a, T>,
    cell_iter: C::PosIter<'a, T>,
}

impl<T: Int, G: Traversal, C: Traversal> IterPosBlock<'_, T, G, C> {
    fn remaining_len(&self) -> usize {
        let remaining_cells = self.cell_iter.size_hint().0;
        let remaining_blocks = self.block_iter.size_hint().0;
        remaining_cells + remaining_blocks * self.layout.size.width * self.layout.size.height
    }
}

impl<T: Int, G: Traversal, C: Traversal> Iterator for IterPosBlock<'_, T, G, C> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cell_iter.next().or_else(|| {
            self.current_block = self.block_iter.next()?;
            self.cell_iter = self.layout.cell.positions(self.current_block);
            self.cell_iter.next()
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining_len();
        (remaining, Some(remaining))
    }
}

impl<T: Int, G: Traversal, C: Traversal> ExactSizeIterator for IterPosBlock<'_, T, G, C> {
    fn len(&self) -> usize {
        self.remaining_len()
    }
}

impl<T: Int, G: Traversal, C: Traversal> FusedIterator for IterPosBlock<'_, T, G, C> {}

/// Iterator over blocks defined by "big" blocks `G` and interior ("small") blocks `C`.
pub struct IterBlockBlock<'a, T: Int, G, C>
where
    G: Traversal,
    C: Traversal,
{
    layout: &'a Block<G, C>,
    current_block: Rect<T>,
    block_iter: G::BlockIter<'a, T>,
    cell_iter: C::PosIter<'a, T>,
}

impl<T: Int, G: Traversal, C: Traversal> IterBlockBlock<'_, T, G, C> {
    fn remaining_len(&self) -> usize {
        let remaining_blocks = self.block_iter.size_hint().0;
        let remaining_cells = self.cell_iter.size_hint().0;
        remaining_blocks * self.layout.size.width * self.layout.size.height + remaining_cells
    }
}

impl<T: Int, G: Traversal, C: Traversal> Iterator for IterBlockBlock<'_, T, G, C> {
    type Item = Rect<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.cell_iter
            .next()
            .map(|pos| Rect::new(pos, self.layout.size))
            .or_else(|| {
                self.current_block = self.block_iter.next()?;
                self.cell_iter = self.layout.cell.positions(self.current_block);
                self.cell_iter
                    .next()
                    .map(|pos| Rect::new(pos, self.layout.size))
            })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining_len();
        (remaining, Some(remaining))
    }
}

impl<T: Int, G: Traversal, C: Traversal> ExactSizeIterator for IterBlockBlock<'_, T, G, C> {
    fn len(&self) -> usize {
        self.remaining_len()
    }
}

impl<T: Int, G: Traversal, C: Traversal> FusedIterator for IterBlockBlock<'_, T, G, C> {}

impl<G: Traversal, C: Traversal> Traversal for Block<G, C> {
    type PosIter<'a, T: Int>
        = IterPosBlock<'a, T, G, C>
    where
        Self: 'a;

    type BlockIter<'a, T: Int>
        = IterBlockBlock<'a, T, G, C>
    where
        Self: 'a;

    /// Returns an iterator over the positions in the specified rectangle.
    ///
    /// The positions are returned in the order defined by the traversal.
    ///
    /// ## Examples
    ///
    /// ```txt
    /// (0, 0) (1, 0) | (2, 0) (3, 0)
    /// (0, 1) (1, 1) | (2, 1) (3, 1)
    /// -----------------------------
    /// (0, 2) (1, 2) | (2, 2) (3, 2)
    /// (0, 3) (1, 3) | (2, 3) (3, 3)
    /// ```
    ///
    /// ```rust
    /// use ixy::{Pos, Rect, Size, layout::{Block, Traversal}};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// let block = Block::row_major(2, 2);
    /// let positions: Vec<_> = block.positions(rect).collect();
    /// assert_eq!(
    ///    positions,
    ///    &[
    ///      // Block 0
    ///      Pos::new(0, 0),
    ///      Pos::new(1, 0),
    ///      Pos::new(0, 1),
    ///      Pos::new(1, 1),
    ///   
    ///      // Block 1
    ///      Pos::new(2, 0),
    ///      Pos::new(3, 0),
    ///      Pos::new(2, 1),
    ///      Pos::new(3, 1),
    ///
    ///      // Block 2
    ///      Pos::new(0, 2),
    ///      Pos::new(1, 2),
    ///      Pos::new(0, 3),
    ///      Pos::new(1, 3),
    ///
    ///      // Block 3
    ///      Pos::new(2, 2),
    ///      Pos::new(3, 2),
    ///      Pos::new(2, 3),
    ///      Pos::new(3, 3),
    ///    ]
    /// );
    /// ```
    fn positions<T: Int>(&self, rect: Rect<T>) -> Self::PosIter<'_, T> {
        let mut block_iter = self.grid.blocks(rect, self.size);
        let current_block = block_iter.next().unwrap_or(Rect::EMPTY);
        let cell_iter = self.cell.positions(current_block);
        IterPosBlock {
            layout: self,
            current_block,
            block_iter,
            cell_iter,
        }
    }

    /// Returns an iterator over (sub-)blocks of the specified size within the rectangle.
    ///
    /// Blocks that would be partially outside the outer or inner rectangles are not yielded.
    ///
    /// ## Examples
    ///
    /// ```txt
    /// OB 0:  OB 1:
    /// • •    • •
    /// • •    • •
    ///
    /// OB 2:  OB 3:
    /// • •    • •
    /// • •    • •
    /// ```
    ///
    /// Where each outer-block looks like this:
    ///
    /// ```txt
    /// +----+----+
    /// | 01 | 45 |
    /// | 23 | 67 |
    /// +----+----+
    /// | 89 | AB |
    /// | CD | EF |
    /// +----+----+
    /// ```
    ///
    /// ```rust
    /// use ixy::{Rect, Size, layout::{Block, Traversal}};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 16, 16);
    /// let outer_block = Block::row_major(4, 4);
    /// let inner_block = Size::new(2, 2);
    /// let blocks: Vec<_> = outer_block.blocks(rect, inner_block).collect();
    ///
    /// assert_eq!(
    ///   blocks,
    ///   &[
    ///     // Outer Block 0
    ///     Rect::from_ltwh(0, 0, 2, 2),
    ///     Rect::from_ltwh(0, 2, 2, 2),
    ///     Rect::from_ltwh(2, 0, 2, 2),
    ///     Rect::from_ltwh(2, 2, 2, 2),
    ///
    ///     // Outer Block 1
    ///     Rect::from_ltwh(4, 0, 2, 2),
    ///     Rect::from_ltwh(4, 2, 2, 2),
    ///     Rect::from_ltwh(6, 0, 2, 2),
    ///     Rect::from_ltwh(6, 2, 2, 2),
    ///
    ///     // Outer Block 2
    ///     Rect::from_ltwh(0, 4, 2, 2),
    ///     Rect::from_ltwh(0, 6, 2, 2),
    ///     Rect::from_ltwh(2, 4, 2, 2),
    ///     Rect::from_ltwh(2, 6, 2, 2),
    ///
    ///     // Outer Block 3
    ///     Rect::from_ltwh(4, 4, 2, 2),
    ///     Rect::from_ltwh(4, 6, 2, 2),
    ///     Rect::from_ltwh(6, 4, 2, 2),
    ///     Rect::from_ltwh(6, 6, 2, 2),
    ///   ]
    /// );
    /// ```
    fn blocks<T: Int>(&self, rect: Rect<T>, size: Size) -> Self::BlockIter<'_, T> {
        let mut block_iter = self.grid.blocks(rect, size);
        let current_block = block_iter.next().unwrap_or(Rect::EMPTY);
        let cell_iter = self.cell.positions(current_block);
        IterBlockBlock {
            layout: self,
            current_block,
            block_iter,
            cell_iter,
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_block_row_major_blocks_row_major_cells_positions() {
        let block = Block::row_major(2, 2);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = block.positions(rect).collect();

        // 0 1 | 2 3
        // 4 5 | 6 7
        // --- | ---
        // 8 9 | A B
        // C D | E F
        assert_eq!(
            positions,
            &[
                // 0 1
                // 2 3
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(0, 1),
                Pos::new(1, 1),
                // 4 5
                // 6 7
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(2, 1),
                Pos::new(3, 1),
                // 8 9
                // A B
                Pos::new(0, 2),
                Pos::new(1, 2),
                Pos::new(0, 3),
                Pos::new(1, 3),
                // C D
                // E F
                Pos::new(2, 2),
                Pos::new(3, 2),
                Pos::new(2, 3),
                Pos::new(3, 3),
            ]
        );
    }

    #[test]
    fn test_block_row_major_big_blocks_row_major_small_blocks() {
        todo!()
    }

    #[test]
    fn test_block_col_major_blocks_col_major_cells_positions() {
        let block = Block::col_major(2, 2);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = block.positions(rect).collect();

        // 0 2 | 8 A
        // 1 3 | 9 B
        // --- | ---
        // 4 6 | C E
        // 5 7 | D F
        assert_eq!(
            positions,
            &[
                // 0 2
                // 1 3
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(1, 0),
                Pos::new(1, 1),
                // 4 6
                // 5 7
                Pos::new(0, 2),
                Pos::new(0, 3),
                Pos::new(1, 2),
                Pos::new(1, 3),
                // 8 A
                // 9 B
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(3, 0),
                Pos::new(3, 1),
                // C E
                // D F
                Pos::new(2, 2),
                Pos::new(2, 3),
                Pos::new(3, 2),
                Pos::new(3, 3),
            ]
        );
    }

    #[test]
    fn test_block_row_major_blocks_col_major_cells_positions() {
        let block = Block::row_major(2, 2).with_cell(ColumnMajor);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = block.positions(rect).collect();

        // 0 2 | 4 6
        // 1 3 | 5 7
        // --- | ---
        // 8 A | C E
        // 9 B | D F
        assert_eq!(
            positions,
            &[
                // 0 2
                // 1 3
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(1, 0),
                Pos::new(1, 1),
                // 4 6
                // 5 7
                Pos::new(2, 0),
                Pos::new(2, 1),
                Pos::new(3, 0),
                Pos::new(3, 1),
                // 8 A
                // 9 B
                Pos::new(0, 2),
                Pos::new(0, 3),
                Pos::new(1, 2),
                Pos::new(1, 3),
                // C E
                // D F
                Pos::new(2, 2),
                Pos::new(2, 3),
                Pos::new(3, 2),
                Pos::new(3, 3),
            ]
        );
    }

    #[test]
    fn test_block_col_major_blocks_row_major_cells_positions() {
        let block = Block::col_major(2, 2).with_cell(RowMajor);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = block.positions(rect).collect();

        // 0 1 | 4 5
        // 2 3 | 6 7
        // --- | ---
        // 4 5 | 8 9
        // 6 7 | A B
        assert_eq!(
            positions,
            &[
                // 0 1
                // 2 3
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(0, 1),
                Pos::new(1, 1),
                // 4 5
                // 6 7
                Pos::new(0, 2),
                Pos::new(1, 2),
                Pos::new(0, 3),
                Pos::new(1, 3),
                // 8 9
                // A B
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(2, 1),
                Pos::new(3, 1),
                // C D
                // E F
                Pos::new(2, 2),
                Pos::new(3, 2),
                Pos::new(2, 3),
                Pos::new(3, 3),
            ]
        );
    }
}
