use crate::{Pos, TryIntoPos, index::Layout};

mod array;
pub use array::ArrayGrid;

mod slice;
pub use slice::SliceGrid;

#[cfg(feature = "alloc")]
mod vec;
#[cfg(feature = "alloc")]
pub use vec::VecGrid;

/// Implementation-level interface for 2-dimensional grids.
///
/// This trait encapsulates the low-level functionality common to all grids; end users should use
/// [`Grid`], which is automatically implemented for every type implementing [`GridCore`].
pub trait GridCore {
    /// The type of element in the grid.
    type Element;

    /// The number of rows in the grid.
    fn width(&self) -> usize;

    /// The number of columns in the grid.
    fn height(&self) -> usize;

    /// Returns a reference to the element at the given position.
    ///
    /// If the position is out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the position is within the bounds of the grid.
    #[allow(unsafe_code)]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element;

    /// Returns a mutable reference to the element at the given position.
    ///
    /// If the position is out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the position is within the bounds of the grid.
    #[allow(unsafe_code)]
    unsafe fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Element;
}

/// Interface for a 2D grid accesible by `Pos<T>`.
pub trait Grid {
    /// The type of element in the grid.
    type Element;

    /// Returns a reference to the element at the given position.
    ///
    /// If the position is out of bounds, returns `None`.
    fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&Self::Element>;

    /// Returns a mutable reference to the element at the given position.
    ///
    /// If the position is out of bounds, returns `None`.
    fn get_mut(&mut self, pos: impl TryIntoPos<usize>) -> Option<&mut Self::Element>;
}

impl<G: GridCore> Grid for G {
    type Element = G::Element;

    fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&Self::Element> {
        let pos = pos.try_into_pos().ok()?;
        if pos.x >= self.width() || pos.y >= self.height() {
            None
        } else {
            #[allow(unsafe_code, reason = "Bounds checked above")]
            Some(unsafe { self.get_unchecked(pos.x, pos.y) })
        }
    }

    fn get_mut(&mut self, pos: impl TryIntoPos<usize>) -> Option<&mut Self::Element> {
        let pos = pos.try_into_pos().ok()?;
        if pos.x >= self.width() || pos.y >= self.height() {
            None
        } else {
            #[allow(unsafe_code, reason = "Bounds checked above")]
            Some(unsafe { self.get_mut_unchecked(pos.x, pos.y) })
        }
    }
}

/// A grid that is backed by a linear memory layout.
pub trait LinearGrid {
    type Element;
    type Layout: Layout;

    /// The number of rows in the grid.
    fn width(&self) -> usize;

    /// The number of columns in the grid.
    fn height(&self) -> usize;

    /// Returns the element at the given linear index.
    ///
    /// If the index is out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is within the bounds of the grid.
    #[allow(unsafe_code)]
    unsafe fn get_unchecked(&self, index: usize) -> &Self::Element;

    /// Returns a mutable reference to the element at the given linear index.
    ///
    /// If the index is out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the index is within the bounds of the grid.
    #[allow(unsafe_code)]
    unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Self::Element;
}

impl<G: LinearGrid> GridCore for G {
    type Element = G::Element;

    fn width(&self) -> usize {
        self.width()
    }

    fn height(&self) -> usize {
        self.height()
    }

    #[allow(unsafe_code)]
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element {
        let pos = Pos { x, y };
        let index = G::Layout::to_1d(pos, self.width());
        unsafe { self.get_unchecked(index.index) }
    }

    #[allow(unsafe_code)]
    unsafe fn get_mut_unchecked(&mut self, x: usize, y: usize) -> &mut Self::Element {
        let pos = Pos { x, y };
        let index = G::Layout::to_1d(pos, self.width());
        unsafe { self.get_mut_unchecked(index.index) }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::index::RowMajor;
    use alloc::{vec, vec::Vec};

    struct TestLinearGrid<T> {
        cells: Vec<T>,
        width: usize,
    }

    impl<T> LinearGrid for TestLinearGrid<T> {
        type Element = T;
        type Layout = RowMajor;

        fn width(&self) -> usize {
            self.width
        }

        fn height(&self) -> usize {
            self.cells.len() / self.width
        }

        #[allow(unsafe_code)]
        unsafe fn get_unchecked(&self, index: usize) -> &Self::Element {
            &self.cells[index]
        }

        #[allow(unsafe_code)]
        unsafe fn get_mut_unchecked(&mut self, index: usize) -> &mut Self::Element {
            &mut self.cells[index]
        }
    }

    #[test]
    fn test_linear_grid() {
        let mut grid = TestLinearGrid {
            cells: vec![0; 12],
            width: 4,
        };

        // Set a value
        *grid.get_mut(Pos::new(1, 2)).unwrap() = 42;

        // Get the value
        let value = grid.get(Pos::new(1, 2)).unwrap();
        assert_eq!(*value, 42);
    }

    #[test]
    fn test_invalid_pos_not_valid_usize() {
        let mut grid = TestLinearGrid {
            cells: vec![0; 12],
            width: 4,
        };

        assert!(grid.get(Pos::new(-1, -3)).is_none());
        assert!(grid.get_mut(Pos::new(-1, -3)).is_none());
    }

    #[test]
    fn test_invalid_pos_width_exceeded() {
        let mut grid = TestLinearGrid {
            cells: vec![0; 12],
            width: 4,
        };

        assert!(grid.get(Pos::new(5, 3)).is_none());
        assert!(grid.get_mut(Pos::new(5, 3)).is_none());
    }

    #[test]
    fn test_invalid_pos_height_exceeded() {
        let mut grid = TestLinearGrid {
            cells: vec![0; 12],
            width: 4,
        };

        assert!(grid.get(Pos::new(2, 4)).is_none());
        assert!(grid.get_mut(Pos::new(2, 4)).is_none());
    }
}
