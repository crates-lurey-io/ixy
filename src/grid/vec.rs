extern crate alloc;
use alloc::{vec, vec::Vec};

use core::marker::PhantomData;

use crate::{
    grid::LinearGrid,
    index::{Layout, RowMajor},
};

/// A grid that is backed by a resizable 1-dimensional vector.
#[derive(Debug, Clone)]
pub struct VecGrid<E, L: Layout = RowMajor> {
    data: Vec<E>,
    width: usize,
    layout: PhantomData<L>,
}

impl<E, L: Layout> VecGrid<E, L> {
    /// Creates a new `VecGrid` with the specified width and height.
    ///
    /// The grid is initialized with the given value for all elements.
    #[must_use]
    pub fn new(width: usize, height: usize, value: E) -> Self
    where
        E: Copy,
    {
        let length = width * height;
        let data = vec![value; length];
        Self {
            data,
            width,
            layout: PhantomData,
        }
    }

    /// Creates a new `VecGrid` with the specified width and height.
    ///
    /// The grid is initialized with the default value for all elements.
    #[must_use]
    pub fn with_default(width: usize, height: usize) -> Self
    where
        E: Copy + Default,
    {
        Self::new(width, height, E::default())
    }

    /// Creates a new `VecGrid` from the provided data.
    ///
    /// The `width` must be specified, and the length of the data must match `width * height`.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` is not equal to `width * height`.
    #[must_use]
    pub fn with_cells(width: usize, height: usize, data: impl Into<Vec<E>>) -> Self {
        let data = data.into();
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

impl<E, L: Layout> LinearGrid for VecGrid<E, L> {
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
    fn vec_new() {
        let grid: VecGrid<Tile> = VecGrid::new(3, 2, Tile::Empty);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);

        // Check every element is initialized to `Tile::Empty`
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                assert_eq!(grid.get(Pos::new(x, y)), Some(&Tile::Empty));
            }
        }
    }

    #[test]
    fn vec_with_default() {
        let grid: VecGrid<Tile> = VecGrid::with_default(3, 2);
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);

        // Check every element is initialized to `Tile::Empty`
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                assert_eq!(grid.get(Pos::new(x, y)), Some(&Tile::Empty));
            }
        }
    }

    #[test]
    fn vec_with_cells_ok() {
        let data = vec![Tile::Empty; 6];
        let grid: VecGrid<Tile> = VecGrid::with_cells(2, 3, data);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);

        // Check every element is initialized to `Tile::Empty`
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                assert_eq!(grid.get(Pos::new(x, y)), Some(&Tile::Empty));
            }
        }
    }

    #[test]
    #[should_panic(expected = "data length must be equal to width * height")]
    fn vec_with_cells_panic() {
        // This should panic because data length is not equal to width * height
        let data = vec![Tile::Empty; 5];
        let _grid: VecGrid<Tile> = VecGrid::with_cells(2, 3, data);
    }

    #[test]
    fn get_mut() {
        let mut grid: VecGrid<Tile> = VecGrid::new(2, 2, Tile::Empty);

        // Set a value
        *grid.get_mut(Pos::new(0, 1)).unwrap() = Tile::Piece;

        // Get the value
        let value = grid.get(Pos::new(0, 1)).unwrap();
        assert_eq!(*value, Tile::Piece);
    }
}
