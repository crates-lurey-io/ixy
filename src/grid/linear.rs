use core::marker::PhantomData;

use crate::{
    HasSize, Pos, Size,
    grid::{
        GridError, GridRead, GridReadMut, GridReadMutUnchecked, GridReadUnchecked, GridWrite,
        GridWriteUnchecked, impls,
    },
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
///   grid::GridBuf,
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
/// let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();
///
/// assert_eq!(grid.size(), Size { width: 3, height: 2 });
/// assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));
///
/// grid.set(Pos::new(0, 0), Tile::Wall);
/// assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
/// ```
pub struct GridBuf<E, T, L: Layout = RowMajor> {
    element: PhantomData<E>,
    data: T,
    width: usize,
    height: usize,
    layout: PhantomData<L>,
}

impl<E, T, L> GridBuf<E, T, L>
where
    L: Layout,
{
    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
    }
}

impl<E, T, L> GridBuf<E, T, L>
where
    L: Layout,
    T: AsRef<[E]>,
{
    /// Creates a new `LinearGridBuf` with the given width, height, and existing cells.
    ///
    /// The data is assumed to be indexed in a fashion compatible with the specified [`Layout`].
    ///
    /// # Errors
    ///
    /// If the data's length does not match the expected number of cells (width * height).
    pub fn from_cells(width: usize, height: usize, data: T) -> Result<Self, GridError> {
        let length = width
            .checked_mul(height)
            .ok_or(GridError::InvalidDimensions)?;
        if data.as_ref().len() != length {
            return Err(GridError::InvalidDimensions);
        }
        Ok(Self {
            element: PhantomData,
            data,
            width,
            height,
            layout: PhantomData,
        })
    }

    /// Returns a reference to the element at the given position.
    ///
    /// Returns `None` if the position is out of bounds.
    pub fn get(&self, pos: impl crate::TryIntoPos<usize>) -> Option<&E> {
        GridRead::get(self, pos)
    }
}

impl<E, T, L> GridBuf<E, T, L>
where
    T: AsMut<[E]>,
    L: Layout,
{
    /// Sets the element at the given position to the specified value.
    ///
    /// If the position is out of bounds, this method does nothing.
    pub fn set(&mut self, pos: impl crate::TryIntoPos<usize>, value: E) {
        GridWrite::set(self, pos, value);
    }
}

impl<E, T> GridBuf<E, T, RowMajor>
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
    pub fn from_row_major(width: usize, height: usize, data: T) -> Result<Self, GridError> {
        Self::from_cells(width, height, data)
    }
}

impl<E, T> GridBuf<E, T, ColMajor>
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
    pub fn from_col_major(width: usize, height: usize, data: T) -> Result<Self, GridError> {
        Self::from_cells(width, height, data)
    }
}

impl<E, T, L> HasSize for GridBuf<E, T, L>
where
    L: Layout,
{
    type Dim = usize;

    fn size(&self) -> Size<usize> {
        Size {
            width: self.width,
            height: self.height,
        }
    }
}

impl<E, T, L> GridReadUnchecked for GridBuf<E, T, L>
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

impl<E, T, L> GridRead for GridBuf<E, T, L>
where
    T: AsRef<[E]>,
    L: Layout,
{
    type Element = E;

    fn get(&self, pos: impl crate::TryIntoPos<usize>) -> Option<&<Self as GridRead>::Element> {
        unsafe { impls::get_from_unchecked(self, pos) }
    }
}

impl<E, T, L> GridReadMutUnchecked for GridBuf<E, T, L>
where
    T: AsMut<[E]>,
    L: Layout,
{
    type Element = E;

    unsafe fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Element {
        let index = L::to_1d(Pos::new(x, y), self.width);
        unsafe { self.data.as_mut().get_unchecked_mut(index.index) }
    }
}

impl<E, T, L> GridReadMut for GridBuf<E, T, L>
where
    T: AsMut<[E]>,
    L: Layout,
{
    type Element = E;

    fn get_mut(
        &mut self,
        pos: impl crate::TryIntoPos<usize>,
    ) -> Option<&mut <Self as GridReadMut>::Element> {
        unsafe { impls::get_mut_from_unchecked(self, pos) }
    }
}

impl<E, T, L> GridWriteUnchecked for GridBuf<E, T, L>
where
    T: AsMut<[E]>,
    L: Layout,
{
    type Element = E;

    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Element) {
        let index = L::to_1d(Pos::new(x, y), self.width);
        unsafe { *self.data.as_mut().get_unchecked_mut(index.index) = value }
    }
}

impl<E, T, L> GridWrite for GridBuf<E, T, L>
where
    T: AsMut<[E]>,
    L: Layout,
{
    type Element = E;

    fn set(&mut self, pos: impl crate::TryIntoPos<usize>, value: <Self as GridWrite>::Element) {
        unsafe { impls::set_from_unchecked(self, pos, value) }
    }
}

impl<E, T, L> AsRef<[E]> for GridBuf<E, T, L>
where
    L: Layout,
    T: AsRef<[E]>,
{
    fn as_ref(&self) -> &[E] {
        self.data.as_ref()
    }
}

impl<E, T, L> AsMut<[E]> for GridBuf<E, T, L>
where
    L: Layout,
    T: AsMut<[E]>,
{
    fn as_mut(&mut self) -> &mut [E] {
        self.data.as_mut()
    }
}

impl<E, T, L> IntoIterator for GridBuf<E, T, L>
where
    L: Layout,
    T: AsRef<[E]> + IntoIterator<Item = E>,
{
    type Item = E;
    type IntoIter = T::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::Size;
    use alloc::{vec, vec::Vec};

    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    enum Tile {
        #[default]
        Empty,
        Wall,
    }

    #[test]
    fn impl_invalid_dimensions() {
        // Attempting to create a grid with invalid dimensions should return an error.
        let cells = vec![Tile::Empty; 5];
        let grid = GridBuf::<Tile, Vec<Tile>>::from_row_major(3, 2, cells);
        assert!(grid.is_err());
    }

    #[test]
    fn impl_vec() {
        // Using a vec as a backing-store.
        let cells = vec![Tile::Empty; 6];
        let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();

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
        let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();

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
        let mut grid = GridBuf::from_row_major(3, 2, &mut cells).unwrap();

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
    fn col_major() {
        // Using a vec as a backing-store and iterating over it in column-major order.
        #[rustfmt::skip]
        let cells = vec![
            Tile::Empty, Tile::Wall,
            Tile::Wall,  Tile::Empty,
        ];

        let grid = GridBuf::from_col_major(2, 2, cells).unwrap();
        let mut iter = grid.into_iter();
        assert_eq!(iter.next(), Some(Tile::Empty));
        assert_eq!(iter.next(), Some(Tile::Wall));
        assert_eq!(iter.next(), Some(Tile::Wall));
        assert_eq!(iter.next(), Some(Tile::Empty));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn as_ref() {
        let cells = vec![Tile::Empty; 6];
        let grid = GridBuf::from_row_major(3, 2, cells).unwrap();
        let slice: &[Tile] = grid.as_ref();
        assert_eq!(slice.len(), 6);
    }

    #[test]
    fn as_mut() {
        let cells = vec![Tile::Empty; 6];
        let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();
        let slice: &mut [Tile] = grid.as_mut();
        assert_eq!(slice.len(), 6);
    }

    #[test]
    fn into_iter() {
        // Using a vec as a backing-store and iterating over it.
        #[rustfmt::skip]
        let cells = vec![
            Tile::Empty, Tile::Wall,
            Tile::Wall,  Tile::Empty,
        ];

        let grid = GridBuf::from_row_major(2, 2, cells).unwrap();
        let mut iter = grid.into_iter();
        assert_eq!(iter.next(), Some(Tile::Empty));
        assert_eq!(iter.next(), Some(Tile::Wall));
        assert_eq!(iter.next(), Some(Tile::Wall));
        assert_eq!(iter.next(), Some(Tile::Empty));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn size() {
        let cells = vec![Tile::Empty; 6];
        let grid = GridBuf::from_row_major(3, 2, cells).unwrap();
        assert_eq!(grid.width(), 3);
        assert_eq!(grid.height(), 2);
    }

    #[test]
    fn get_mut() {
        let cells = vec![Tile::Empty; 6];
        let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();

        assert_eq!(grid.get_mut(Pos::new(0, 0)), Some(&mut Tile::Empty));
    }

    #[test]
    fn get_mut_unchecked() {
        let cells = vec![Tile::Empty; 6];
        let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();

        unsafe {
            assert_eq!(grid.get_mut_unchecked(0, 0), &mut Tile::Empty);
            *grid.get_mut_unchecked(0, 0) = Tile::Wall;
        }

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
    }
}
