use core::marker::PhantomData;

use crate::{
    grid::LinearGrid,
    index::{Layout, RowMajor},
};

/// A grid that is backed by a fixed-size 1-dimensional slice.
#[derive(Debug)]
pub struct SliceGrid<'a, E, L: Layout = RowMajor> {
    data: &'a mut [E],
    width: usize,
    layout: PhantomData<L>,
}

impl<'a, E, L: Layout> SliceGrid<'a, E, L> {
    /// Creates a new `SliceGrid` from the provided slice.
    ///
    /// The `width` must be specified, and the length of the slice must match `width * height`.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` is not equal to `width * height`.
    pub fn with_cells(width: usize, height: usize, data: &'a mut [E]) -> Self {
        assert_eq!(
            width * height,
            data.len(),
            "data length must be equal to width * height"
        );
        Self {
            data,
            width,
            layout: PhantomData,
        }
    }
}

impl<E, L: Layout> LinearGrid for SliceGrid<'_, E, L> {
    type Element = E;
    type Layout = L;

    fn width(&self) -> usize {
        self.width
    }

    fn height(&self) -> usize {
        self.data.len() / self.width
    }

    #[allow(unsafe_code)]
    unsafe fn get_unchecked(&self, index: usize) -> &Self::Element {
        &self.data[index]
    }

    #[allow(unsafe_code)]
    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Self::Element {
        &mut self.data[index]
    }
}

#[cfg(test)]
mod tests {
    use crate::{Grid, Pos};

    use super::*;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        #[default]
        Empty,
        Piece,
    }

    #[test]
    fn slice_ok() {
        let mut data = [Tile::Empty; 6];
        let grid = SliceGrid::<_>::with_cells(2, 3, &mut data);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);

        // Check every element is initialized to `Tile::Empty`
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = Pos::new(x, y);
                assert_eq!(grid.get(pos), Some(&Tile::Empty));
            }
        }
    }

    #[test]
    #[should_panic(expected = "data length must be equal to width * height")]
    fn slice_from_panic() {
        // This should panic because data length is not equal to width * height
        let mut data = [Tile::Empty; 5];
        let _grid = SliceGrid::<_>::with_cells(2, 3, &mut data);
    }

    #[test]
    fn get_mut() {
        let mut data = [Tile::Empty; 6];
        let mut grid = SliceGrid::<_>::with_cells(2, 3, &mut data);

        // Modify a cell
        let pos = Pos::new(1, 1);
        *grid.get_mut(pos).unwrap() = Tile::Piece;

        // Check the modification
        assert_eq!(grid.get(pos), Some(&Tile::Piece));
    }
}
