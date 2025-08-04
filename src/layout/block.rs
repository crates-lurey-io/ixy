use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{ColumnMajor, Linear, RowMajor, Traversal},
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
/// let cells: Vec<_> = block.pos_iter(rect).collect();
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

impl<G: Traversal, C: Traversal> Traversal for Block<G, C> {
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
    /// let positions: Vec<_> = block.pos_iter(rect).collect();
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
    fn iter_pos<T: Int>(&self, rect: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        self.grid
            .iter_block(rect, self.size)
            .flat_map(move |block_rect| self.cell.iter_pos(block_rect))
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
    /// let rect = Rect::from_ltwh(0, 0, 8, 8);
    /// let outer_block = Block::row_major(4, 4);
    /// let inner_block = Size::new(2, 2);
    /// let blocks: Vec<_> = outer_block.rect_iter(rect, inner_block).collect();
    ///
    /// assert_eq!(
    ///   blocks,
    ///   &[
    ///     // Outer Block 0
    ///     Rect::from_ltwh(0, 0, 2, 2),
    ///     Rect::from_ltwh(2, 0, 2, 2),
    ///     Rect::from_ltwh(0, 2, 2, 2),
    ///     Rect::from_ltwh(2, 2, 2, 2),
    ///
    ///     // Outer Block 1
    ///     Rect::from_ltwh(4, 0, 2, 2),
    ///     Rect::from_ltwh(6, 0, 2, 2),
    ///     Rect::from_ltwh(4, 2, 2, 2),
    ///     Rect::from_ltwh(6, 2, 2, 2),
    ///
    ///     // Outer Block 2
    ///     Rect::from_ltwh(0, 4, 2, 2),
    ///     Rect::from_ltwh(2, 4, 2, 2),
    ///     Rect::from_ltwh(0, 6, 2, 2),
    ///     Rect::from_ltwh(2, 6, 2, 2),
    ///
    ///     // Outer Block 3
    ///     Rect::from_ltwh(4, 4, 2, 2),
    ///     Rect::from_ltwh(6, 4, 2, 2),
    ///     Rect::from_ltwh(4, 6, 2, 2),
    ///     Rect::from_ltwh(6, 6, 2, 2),
    ///   ]
    /// );
    /// ```
    fn iter_block<T: Int>(&self, rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>> {
        self.grid
            .iter_block(rect, self.size)
            .flat_map(move |block_rect| self.cell.iter_block(block_rect, size))
    }
}

impl<G: Traversal, C: Traversal> Linear for Block<G, C>
where
    G: Linear,
    C: Linear,
{
    fn pos_to_index(&self, pos: Pos<usize>, width: usize) -> usize {
        let block_x = pos.x.to_usize() / self.size.width;
        let block_y = pos.y.to_usize() / self.size.height;
        let cell_x = pos.x.to_usize() % self.size.width;
        let cell_y = pos.y.to_usize() % self.size.height;

        let block_pos = Pos::new(block_x, block_y);
        let cell_pos = Pos::new(cell_x, cell_y);

        let block_offset = self.grid.pos_to_index(block_pos, width / self.size.width);
        let cell_offset = self.cell.pos_to_index(cell_pos, self.size.width);

        block_offset * (self.size.width * self.size.height) + cell_offset
    }

    fn index_to_pos(&self, index: usize, width: usize) -> Pos<usize> {
        let cells_per_block = self.size.width * self.size.height;
        let block_index = index / cells_per_block;
        let cell_index = index % cells_per_block;

        let block_grid_width = width / self.size.width;
        let block_pos = self.grid.index_to_pos(block_index, block_grid_width);
        let cell_pos = self.cell.index_to_pos(cell_index, self.size.width);

        block_pos * self.size.to_pos() + cell_pos
    }

    fn len_aligned(&self, size: Size) -> usize {
        self.grid.len_aligned(size)
    }

    fn slice_aligned<'a, E>(&self, slice: &'a [E], size: Size, axis: usize) -> &'a [E] {
        self.grid.slice_aligned(slice, size, axis)
    }

    fn slice_aligned_mut<'a, E>(&self, slice: &'a mut [E], size: Size, axis: usize) -> &'a mut [E] {
        self.grid.slice_aligned_mut(slice, size, axis)
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
        let positions: Vec<_> = block.iter_pos(rect).collect();

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
        let rect = Rect::from_ltwh(0, 0, 16, 16);
        let block = Block::row_major(4, 4);
        let size = Size::new(2, 2);
        let blocks: Vec<_> = block.iter_block(rect, size).collect();
        assert_eq!(
            blocks,
            &[
                // Outer Block 0
                // Inner Block 0
                Rect::from_ltwh(0, 0, 2, 2),
                Rect::from_ltwh(2, 0, 2, 2),
                Rect::from_ltwh(0, 2, 2, 2),
                Rect::from_ltwh(2, 2, 2, 2),
                // Inner Block 1
                Rect::from_ltwh(4, 0, 2, 2),
                Rect::from_ltwh(6, 0, 2, 2),
                Rect::from_ltwh(4, 2, 2, 2),
                Rect::from_ltwh(6, 2, 2, 2),
                // Inner Block 2
                Rect::from_ltwh(8, 0, 2, 2),
                Rect::from_ltwh(10, 0, 2, 2),
                Rect::from_ltwh(8, 2, 2, 2),
                Rect::from_ltwh(10, 2, 2, 2),
                // Inner Block 3
                Rect::from_ltwh(12, 0, 2, 2),
                Rect::from_ltwh(14, 0, 2, 2),
                Rect::from_ltwh(12, 2, 2, 2),
                Rect::from_ltwh(14, 2, 2, 2),
                // Outer Block 1
                // Inner Block 4
                Rect::from_ltwh(0, 4, 2, 2),
                Rect::from_ltwh(2, 4, 2, 2),
                Rect::from_ltwh(0, 6, 2, 2),
                Rect::from_ltwh(2, 6, 2, 2),
                // Inner Block 5
                Rect::from_ltwh(4, 4, 2, 2),
                Rect::from_ltwh(6, 4, 2, 2),
                Rect::from_ltwh(4, 6, 2, 2),
                Rect::from_ltwh(6, 6, 2, 2),
                // Inner Block 6
                Rect::from_ltwh(8, 4, 2, 2),
                Rect::from_ltwh(10, 4, 2, 2),
                Rect::from_ltwh(8, 6, 2, 2),
                Rect::from_ltwh(10, 6, 2, 2),
                // Inner Block 7
                Rect::from_ltwh(12, 4, 2, 2),
                Rect::from_ltwh(14, 4, 2, 2),
                Rect::from_ltwh(12, 6, 2, 2),
                Rect::from_ltwh(14, 6, 2, 2),
                // Outer Block 2
                // Inner Block 8
                Rect::from_ltwh(0, 8, 2, 2),
                Rect::from_ltwh(2, 8, 2, 2),
                Rect::from_ltwh(0, 10, 2, 2),
                Rect::from_ltwh(2, 10, 2, 2),
                // Inner Block 9
                Rect::from_ltwh(4, 8, 2, 2),
                Rect::from_ltwh(6, 8, 2, 2),
                Rect::from_ltwh(4, 10, 2, 2),
                Rect::from_ltwh(6, 10, 2, 2),
                // Inner Block A
                Rect::from_ltwh(8, 8, 2, 2),
                Rect::from_ltwh(10, 8, 2, 2),
                Rect::from_ltwh(8, 10, 2, 2),
                Rect::from_ltwh(10, 10, 2, 2),
                // Inner Block B
                Rect::from_ltwh(12, 8, 2, 2),
                Rect::from_ltwh(14, 8, 2, 2),
                Rect::from_ltwh(12, 10, 2, 2),
                Rect::from_ltwh(14, 10, 2, 2),
                // Outer Block 3
                // Inner Block C
                Rect::from_ltwh(0, 12, 2, 2),
                Rect::from_ltwh(2, 12, 2, 2),
                Rect::from_ltwh(0, 14, 2, 2),
                Rect::from_ltwh(2, 14, 2, 2),
                // Inner Block D
                Rect::from_ltwh(4, 12, 2, 2),
                Rect::from_ltwh(6, 12, 2, 2),
                Rect::from_ltwh(4, 14, 2, 2),
                Rect::from_ltwh(6, 14, 2, 2),
                // Inner Block E
                Rect::from_ltwh(8, 12, 2, 2),
                Rect::from_ltwh(10, 12, 2, 2),
                Rect::from_ltwh(8, 14, 2, 2),
                Rect::from_ltwh(10, 14, 2, 2),
                // Inner Block F
                Rect::from_ltwh(12, 12, 2, 2),
                Rect::from_ltwh(14, 12, 2, 2),
                Rect::from_ltwh(12, 14, 2, 2),
                Rect::from_ltwh(14, 14, 2, 2),
            ]
        );
    }

    #[test]
    fn test_block_col_major_blocks_col_major_cells_positions() {
        let block = Block::col_major(2, 2);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = block.iter_pos(rect).collect();

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
        let positions: Vec<_> = block.iter_pos(rect).collect();

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
        let positions: Vec<_> = block.iter_pos(rect).collect();

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

    #[test]
    fn test_block_row_major_to_1d() {
        // 0 1 | 4 5
        // 2 3 | 6 7
        // --- | ---
        // 8 9 | A B
        // C D | E F
        let block = Block::row_major(4, 4);
        let expected: Vec<_> = (0..16).collect();
        let actual: Vec<_> = (0..4)
            .flat_map(|y| (0..4).map(move |x| block.pos_to_index(Pos::new(x, y), 4)))
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_block_col_major_to_1d() {
        // 0 2 | 8 A
        // 1 3 | 9 B
        // --- | ---
        // 4 6 | C E
        // 5 7 | D F
        let block = Block::col_major(4, 4);
        let expected: Vec<_> = (0..16).collect();
        let actual: Vec<_> = (0..4)
            .flat_map(|x| (0..4).map(move |y| block.pos_to_index(Pos::new(x, y), 4)))
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_block_row_major_to_2d() {
        // 0 1 | 4 5
        // 2 3 | 6 7
        // --- | ---
        // 8 9 | A B
        // C D | E F
        let block = Block::row_major(4, 4);
        let expected: Vec<_> = (0..4)
            .flat_map(|y| (0..4).map(move |x| Pos::new(x, y)))
            .collect();
        let actual: Vec<_> = (0..16).map(|i| block.index_to_pos(i, 4)).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn block_new() {
        let block = Block::new(3, 4, RowMajor, ColumnMajor);
        assert_eq!(block.size.width, 3);
        assert_eq!(block.size.height, 4);
        assert_eq!(block.grid, RowMajor);
        assert_eq!(block.cell, ColumnMajor);
    }

    #[test]
    fn block_with_grid() {
        let block = Block::row_major(2, 2).with_grid(ColumnMajor);
        assert_eq!(block.size.width, 2);
        assert_eq!(block.size.height, 2);
        assert_eq!(block.grid, ColumnMajor);
        assert_eq!(block.cell, RowMajor);
    }

    #[test]
    fn len_aligned() {
        let block = Block::row_major(2, 2);
        let size = Size::new(4, 2);
        assert_eq!(block.len_aligned(size), 2);
    }

    #[test]
    fn slice_aligned_mut_in_bounds() {
        #[rustfmt::skip]
        let slice = &mut [
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let block = Block::row_major(2, 2);
        let size = Size::new(4, 2);
        assert_eq!(block.slice_aligned_mut(slice, size, 0), &mut [0, 1, 2, 3]);
    }

    #[test]
    fn slice_aligned_in_bounds() {
        #[rustfmt::skip]
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let block = Block::row_major(2, 2);
        let size = Size::new(4, 2);
        assert_eq!(block.slice_aligned(slice, size, 0), &[0, 1, 2, 3]);
    }

    #[test]
    fn slice_aligned_out_of_bounds() {
        #[rustfmt::skip]    
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let block = Block::row_major(2, 2);
        let size = Size::new(4, 2);
        assert_eq!(block.slice_aligned(slice, size, 2), &[]);
    }
}
