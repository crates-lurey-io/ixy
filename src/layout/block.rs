use core::{marker::PhantomData, ops::Range};

use crate::{
    Pos, Rect, Size,
    int::Int,
    layout::{Linear, RowMajor, Traversal},
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
/// For example, `Block<2, 2, RowMajor, RowMajor>` (a block-size of 2x2, with row-major traversals):
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
/// let cells: Vec<_> = Block::<2, 2>::iter_pos(rect).collect();
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
pub struct Block<const W: usize, const H: usize, G = RowMajor, C = G> {
    grid: PhantomData<G>,
    cell: PhantomData<C>,
}

impl<const W: usize, const H: usize, G: Traversal, C: Traversal> Traversal for Block<W, H, G, C> {
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
    /// let positions: Vec<_> = Block::<2, 2>::iter_pos(rect).collect();
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
    fn iter_pos<T: Int>(rect: Rect<T>) -> impl Iterator<Item = Pos<T>> {
        G::iter_rect(
            rect,
            Size {
                width: W,
                height: H,
            },
        )
        .flat_map(move |block_rect| C::iter_pos(block_rect))
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
    /// let inner_block = Size::new(2, 2);
    /// let blocks: Vec<_> = Block::<4, 4>::iter_rect(rect, inner_block).collect();
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
    fn iter_rect<T: Int>(rect: Rect<T>, size: Size) -> impl Iterator<Item = Rect<T>> {
        G::iter_rect(
            rect,
            Size {
                width: W,
                height: H,
            },
        )
        .flat_map(move |block_rect| C::iter_rect(block_rect, size))
    }
}

impl<const W: usize, const H: usize, G: Traversal, C: Traversal> Linear for Block<W, H, G, C>
where
    G: Linear,
    C: Linear,
{
    fn pos_to_index(pos: Pos<usize>, width: usize) -> usize {
        let block_x = pos.x / W;
        let block_y = pos.y / H;
        let cell_x = pos.x % W;
        let cell_y = pos.y % H;

        let block_pos = Pos::new(block_x, block_y);
        let cell_pos = Pos::new(cell_x, cell_y);

        let blocks_per_row = width / W;
        let block_offset = G::pos_to_index(block_pos, blocks_per_row);
        let cell_offset = C::pos_to_index(cell_pos, W);

        block_offset * (W * H) + cell_offset
    }

    fn index_to_pos(index: usize, width: usize) -> Pos<usize> {
        let cells_per_block = W * H;
        let block_index = index / cells_per_block;
        let cell_index = index % cells_per_block;

        let block_grid_width = width / W;
        let block_pos = G::index_to_pos(block_index, block_grid_width);
        let cell_pos = C::index_to_pos(cell_index, W);

        block_pos * Pos::new(W, H) + cell_pos
    }

    fn len_aligned(size: Size) -> usize {
        G::len_aligned(size)
    }

    fn rect_to_range(grid_size: Size, rect: Rect<usize>) -> Option<Range<usize>> {
        // Must be either:
        // - Elements entirely within a single block
        // - Elements spanning multiple blocks but full-sized

        // Check if the rectangle is aligned to the block size
        if rect.width() % W != 0 || rect.height() % H != 0 {
            return None;
        }

        // Calculate the start and end indices based on the block layout
        let start = Self::pos_to_index(rect.top_left(), grid_size.width);
        let end = Self::pos_to_index(rect.bottom_right() - Pos::new(1, 1), grid_size.width) + 1;
        if end > grid_size.width * grid_size.height {
            return None;
        }

        Some(start..end)
    }

    fn slice_rect_aligned<E>(slice: &[E], size: Size, rect: Rect<usize>) -> Option<&[E]> {
        let range = Self::rect_to_range(size, rect)?;
        if range.end > slice.len() {
            return None;
        }
        Some(&slice[range])
    }

    fn slice_rect_aligned_mut<E>(
        slice: &mut [E],
        size: Size,
        rect: Rect<usize>,
    ) -> Option<&mut [E]> {
        let range = Self::rect_to_range(size, rect)?;
        if range.end > slice.len() {
            return None;
        }
        Some(&mut slice[range])
    }

    fn slice_aligned<E>(slice: &[E], size: Size, axis: usize) -> &[E] {
        G::slice_aligned(slice, size, axis)
    }

    fn slice_aligned_mut<E>(slice: &mut [E], size: Size, axis: usize) -> &mut [E] {
        G::slice_aligned_mut(slice, size, axis)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::layout::ColumnMajor;

    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn test_block_row_major_blocks_row_major_cells_positions() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = Block::<2, 2>::iter_pos(rect).collect();

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
        let size = Size::new(2, 2);
        let blocks: Vec<_> = Block::<4, 4>::iter_rect(rect, size).collect();
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
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = Block::<2, 2, ColumnMajor>::iter_pos(rect).collect();

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
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = Block::<2, 2, RowMajor, ColumnMajor>::iter_pos(rect).collect();

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
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        let positions: Vec<_> = Block::<2, 2, ColumnMajor, RowMajor>::iter_pos(rect).collect();

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
        let expected: Vec<_> = (0..16).collect();
        let actual: Vec<_> = (0..4)
            .flat_map(|y| (0..4).map(move |x| Block::<4, 4>::pos_to_index(Pos::new(x, y), 4)))
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
        let expected: Vec<_> = (0..16).collect();
        let actual: Vec<_> = (0..4)
            .flat_map(|x| {
                (0..4).map(move |y| Block::<4, 4, ColumnMajor>::pos_to_index(Pos::new(x, y), 4))
            })
            .collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_pos_to_index() {
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(0, 0), 4), 0);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(1, 0), 4), 1);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(0, 1), 4), 2);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(1, 1), 4), 3);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(2, 0), 4), 4);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(3, 0), 4), 5);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(2, 1), 4), 6);
        assert_eq!(Block::<2, 2>::pos_to_index(Pos::new(3, 1), 4), 7);
    }

    #[test]
    fn test_block_row_major_to_2d() {
        // 0 1 | 4 5
        // 2 3 | 6 7
        // --- | ---
        // 8 9 | A B
        // C D | E F
        let expected: Vec<_> = (0..4)
            .flat_map(|y| (0..4).map(move |x| Pos::new(x, y)))
            .collect();
        let actual: Vec<_> = (0..16).map(|i| Block::<4, 4>::index_to_pos(i, 4)).collect();
        assert_eq!(actual, expected);
    }

    #[test]
    fn len_aligned() {
        let size = Size::new(4, 2);
        assert_eq!(Block::<2, 2>::len_aligned(size), 2);
    }

    #[test]
    fn slice_aligned_mut_in_bounds() {
        #[rustfmt::skip]
        let slice = &mut [
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(
            Block::<2, 2>::slice_aligned_mut(slice, size, 0),
            &mut [0, 1, 2, 3]
        );
    }

    #[test]
    fn slice_aligned_in_bounds() {
        #[rustfmt::skip]
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(Block::<2, 2>::slice_aligned(slice, size, 0), &[0, 1, 2, 3]);
    }

    #[test]
    fn slice_aligned_out_of_bounds() {
        #[rustfmt::skip]    
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(Block::<2, 2>::slice_aligned(slice, size, 2), &[]);
    }

    #[test]
    fn slice_rect_aligned_full() {
        #[rustfmt::skip]
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(
            Block::<2, 2>::slice_rect_aligned(slice, size, Rect::from_ltwh(0, 0, 4, 2)),
            Some(&[0, 1, 2, 3, 4, 5, 6, 7][..])
        );
    }

    #[test]
    fn slice_rect_aligned_partial() {
        #[rustfmt::skip]
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(
            Block::<2, 2>::slice_rect_aligned(slice, size, Rect::from_ltwh(0, 0, 2, 2)),
            Some(&[0, 1, 2, 3][..])
        );
        assert_eq!(
            Block::<2, 2>::slice_rect_aligned(slice, size, Rect::from_ltwh(2, 0, 2, 2)),
            Some(&[4, 5, 6, 7][..])
        );
    }

    #[test]
    fn slice_rect_aligned_out_of_bounds() {
        #[rustfmt::skip]
        let slice = &[
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(
            Block::<2, 2>::slice_rect_aligned(slice, size, Rect::from_ltwh(0, 0, 5, 2)),
            None
        );
    }

    #[test]
    fn slice_rect_unaligned_partial() {
        #[rustfmt::skip]
        let slice = &mut [
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        let rect = Rect::from_ltwh(0, 1, 1, 2);
        assert_eq!(Block::<2, 2>::slice_rect_aligned_mut(slice, size, rect), None);
    }

    #[test]
    fn slice_rect_aligned_mut_full() {
        #[rustfmt::skip]
        let slice = &mut [
            0, 1, 2, 3,
            4, 5, 6, 7,
        ];
        let size = Size::new(4, 2);
        assert_eq!(
            Block::<2, 2>::slice_rect_aligned_mut(slice, size, Rect::from_ltwh(0, 0, 4, 2)),
            Some(&mut [0, 1, 2, 3, 4, 5, 6, 7][..])
        );
    }
}
