use core::marker::PhantomData;

use crate::{
    grid::LinearGrid,
    index::{Layout, RowMajor},
};

/// A grid that is backed by a fixed-size 1-dimensional array.
#[derive(Debug, Clone)]
pub struct ArrayGrid<E, const LENGTH: usize, L: Layout = RowMajor> {
    data: [E; LENGTH],
    width: usize,
    layout: PhantomData<L>,
}

impl<E, const LENGTH: usize, L: Layout> ArrayGrid<E, LENGTH, L> {
    /// Creates a new `ArrayGrid` with the specified width and height.
    ///
    /// The grid is initialized with the given value for all elements.
    ///
    /// # Panics
    ///
    /// Panics if `LENGTH` is not equal to `width * height`.
    #[must_use]
    pub fn new(width: usize, height: usize, value: E) -> Self
    where
        E: Copy,
    {
        assert_eq!(
            width * height,
            LENGTH,
            "LENGTH must be equal to width * height"
        );
        let data = [value; LENGTH];
        Self {
            data,
            width,
            layout: PhantomData,
        }
    }

    /// Creates a new `ArrayGrid` with the specified width and height.
    ///
    /// The grid is initialized with the default value for all elements.
    ///
    /// # Panics
    ///
    /// Panics if `LENGTH` is not equal to `width * height`.
    #[must_use]
    pub fn with_default(width: usize, height: usize) -> Self
    where
        E: Copy + Default,
    {
        Self::new(width, height, E::default())
    }

    /// Creates a new `ArrayGrid` from the provided data.
    ///
    /// The `width` must be specified, and the length of the data must match `width * height`.
    ///
    /// # Panics
    ///
    /// Panics if `data.len()` is not equal to `width * height`.
    #[must_use]
    pub fn with_cells(width: usize, height: usize, data: impl Into<[E; LENGTH]>) -> Self {
        let data = data.into();
        assert_eq!(
            width * height,
            LENGTH,
            "LENGTH must be equal to width * height"
        );
        Self {
            data,
            width,
            layout: PhantomData,
        }
    }
}

impl<E, const LENGTH: usize, L: Layout> LinearGrid for ArrayGrid<E, LENGTH, L> {
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
    fn array_new_ok() {
        let grid: ArrayGrid<Tile, 6> = ArrayGrid::new(2, 3, Tile::Empty);
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
    #[should_panic(expected = "LENGTH must be equal to width * height")]
    fn array_new_panic() {
        // This should panic because LENGTH is not equal to width * height
        let _grid: ArrayGrid<Tile, 5> = ArrayGrid::new(2, 3, Tile::Empty);
    }

    #[test]
    fn array_with_default_ok() {
        let grid: ArrayGrid<Tile, 6> = ArrayGrid::with_default(2, 3);
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
    #[should_panic(expected = "LENGTH must be equal to width * height")]
    fn array_with_default_panic() {
        // This should panic because LENGTH is not equal to width * height
        let _grid: ArrayGrid<Tile, 5> = ArrayGrid::with_default(2, 3);
    }

    #[test]
    fn array_from_ok() {
        let data = [
            Tile::Empty,
            Tile::Piece,
            Tile::Empty,
            Tile::Piece,
            Tile::Empty,
            Tile::Piece,
        ];
        let grid: ArrayGrid<Tile, 6> = ArrayGrid::with_cells(2, 3, data);
        assert_eq!(grid.width(), 2);
        assert_eq!(grid.height(), 3);

        // Check every element matches the data
        for y in 0..grid.height() {
            for x in 0..grid.width() {
                let pos = Pos::new(x, y);
                assert_eq!(grid.get(pos), Some(&data[y * grid.width() + x]));
            }
        }
    }

    #[test]
    #[should_panic(expected = "LENGTH must be equal to width * height")]
    fn array_from_panic() {
        // This should panic because data length is not equal to LENGTH
        let data = [Tile::Empty; 6];
        let _grid: ArrayGrid<Tile, 6> = ArrayGrid::with_cells(4, 3, data);
    }

    #[test]
    fn get_mut() {
        let mut grid: ArrayGrid<Tile, 6> = ArrayGrid::with_default(2, 3);
        let pos = Pos::new(1, 2);
        assert_eq!(grid.get(pos), Some(&Tile::Empty));

        // Modify the element at (1, 2)
        *grid.get_mut(pos).unwrap() = Tile::Piece;

        // Verify the change
        assert_eq!(grid.get(pos), Some(&Tile::Piece));
    }
}
