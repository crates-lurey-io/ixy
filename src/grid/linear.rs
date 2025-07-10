use core::marker::PhantomData;

use crate::{
    HasSize, Pos,
    grid::{GridReadUnchecked, GridWriteUnchecked},
    index::{ColMajor, Layout, RowMajor},
};

/// A grid that is backed by a linear representation in memory.
///
/// Any type that implements `AsRef<[E]>` can be used as the backing store for the grid, and the
/// elements are indexed according to the specified [`Layout`]. For example, a `Vec<E>`, an array
/// `[E; N]`, or a slice `&[E]` (or `&mut [E]`) can be used.
///
/// # Examples
///
/// ```rust
/// use ixy::{
///   grid::{GridRead, GridWrite, LinearGridBuf},
///   HasSize,
///   Pos,
///   Size,
/// };
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Tile {
///   Empty,
///   Wall,
/// }
///
/// let cells = vec![Tile::Empty; 6];
/// let mut grid = LinearGridBuf::from_row_major(3, 2, cells).unwrap();
///
/// assert_eq!(grid.size(), Size { width: 3, height: 2 });
/// assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));
///
/// grid.set(Pos::new(0, 0), Tile::Wall);
/// assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
/// ```
pub struct LinearGridBuf<E, T: AsRef<[E]>, L: Layout = RowMajor> {
    element: PhantomData<E>,
    data: T,
    width: usize,
    layout: PhantomData<L>,
}

/// An error that can occur when creating a `LinearGrid`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LinearGridError {
    /// The dimensions of the grid are invalid compared to the data provided.
    InvalidDimensions,
}

impl<E, T, L> LinearGridBuf<E, T, L>
where
    T: AsRef<[E]>,
    L: Layout,
{
    /// Creates a new `LinearGridBuf` with the given width, height, and existing cells.
    ///
    /// The data is assumed to be indexed in a fashion compatible with the specified [`Layout`].
    ///
    /// # Errors
    ///
    /// If the data's length does not match the expected number of cells (width * height).
    pub fn from_cells(width: usize, height: usize, data: T) -> Result<Self, LinearGridError> {
        let length = width
            .checked_mul(height)
            .ok_or(LinearGridError::InvalidDimensions)?;
        if data.as_ref().len() != length {
            return Err(LinearGridError::InvalidDimensions);
        }
        Ok(Self {
            element: PhantomData,
            data,
            width,
            layout: PhantomData,
        })
    }
}

impl<E, T> LinearGridBuf<E, T, RowMajor>
where
    T: AsRef<[E]>,
{
    /// Creates a new `LinearGridBuf` with the given width, height, and existing cells.
    ///
    /// The data is assumed to be indexed in a row-major fashion.
    ///
    /// # Errors
    ///
    /// If the data's length does not match the expected number of cells (width * height).
    pub fn from_row_major(width: usize, height: usize, data: T) -> Result<Self, LinearGridError> {
        Self::from_cells(width, height, data)
    }
}

impl<E, T> LinearGridBuf<E, T, ColMajor>
where
    T: AsRef<[E]>,
{
    /// Creates a new `LinearGridBuf` with the given width, height, and existing cells.
    ///
    /// The data is assumed to be indexed in a column-major fashion.
    ///
    /// # Errors
    ///
    /// If the data's length does not match the expected number of cells (width * height).
    pub fn from_col_major(width: usize, height: usize, data: T) -> Result<Self, LinearGridError> {
        Self::from_cells(width, height, data)
    }
}

impl<E, T, L> HasSize for LinearGridBuf<E, T, L>
where
    T: AsRef<[E]>,
    L: Layout,
{
    type Dim = usize;

    fn size(&self) -> crate::Size<usize> {
        crate::Size {
            width: self.width,
            height: self.data.as_ref().len() / self.width,
        }
    }
}

impl<E, T, L> GridReadUnchecked for LinearGridBuf<E, T, L>
where
    T: AsRef<[E]>,
    L: Layout,
{
    type Element = E;

    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element {
        let index = L::to_1d(Pos::new(x, y), self.width);
        unsafe { self.data.as_ref().get_unchecked(index.index) }
    }
}

impl<E, T, L> GridWriteUnchecked for LinearGridBuf<E, T, L>
where
    T: AsRef<[E]> + AsMut<[E]>,
    L: Layout,
{
    type Element = E;

    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Element) {
        let index = L::to_1d(Pos::new(x, y), self.width);
        unsafe { *self.data.as_mut().get_unchecked_mut(index.index) = value }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::{
        Size,
        grid::{GridRead, GridWrite},
    };
    use alloc::vec;

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        #[default]
        Empty,
        Wall,
    }

    #[test]
    fn impl_vec() {
        // Using a vec as a backing-store.
        let cells = vec![Tile::Empty; 6];
        let mut grid = LinearGridBuf::from_row_major(3, 2, cells).unwrap();

        assert_eq!(
            grid.size(),
            Size {
                width: 3,
                height: 2
            }
        );

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));

        grid.set(Pos::new(0, 0), Tile::Wall);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
    }

    #[test]
    fn impl_array() {
        // Using an array as a backing-store.
        let cells = [Tile::Empty; 6];
        let mut grid = LinearGridBuf::from_row_major(3, 2, cells).unwrap();

        assert_eq!(
            grid.size(),
            Size {
                width: 3,
                height: 2
            }
        );

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));

        grid.set(Pos::new(0, 0), Tile::Wall);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
    }

    #[test]
    fn impl_slice() {
        // Using a mutable slice as a backing-store.
        let mut cells = [Tile::Empty; 6];
        let mut grid = LinearGridBuf::from_row_major(3, 2, &mut cells).unwrap();

        assert_eq!(
            grid.size(),
            Size {
                width: 3,
                height: 2
            }
        );

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));

        grid.set(Pos::new(0, 0), Tile::Wall);
        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
    }
}
