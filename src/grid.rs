//! Provides a standard interface for 2D grid-like data structures.
//!
//! A default implementation, based on linear (1-dimensional) memory, is provided as [`GridBuf`].
//!
//! Cheap views into existing grids can be created using [`GridView`] and [`GridViewMut`].
//!
//! Low-level interfaces are provided for interopability:
//! - [`GridRead`], analogous to the `Read` trait, for reading elements from a grid-like structure.
//! - [`GridWrite`], analogous to the `Write` trait, for writing elements to a grid-like structure.
//!
//! Performance sensitive code can consider using the unchecked (`unsafe`) variants:
//! - [`GridReadUnchecked`], for unchecked read access to elements in a grid-like structure.
//! - [`GridWriteUnchecked`], for unchecked write access to elements in a grid-like structure.
//!
//! For additional functionality, see [`GridReadExt`] and [`GridWriteExt`].
//!
//! # Examples
//!
//! ```rust
//! use ixy::{HasSize, Pos, Size, grid::GridBuf};
//!
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum Tile {
//!   Empty,
//!   Wall,
//! }
//!
//! let cells = vec![Tile::Empty; 6];
//! let mut grid = GridBuf::from_row_major(3, 2, cells).unwrap();
//!
//! assert_eq!(grid.size(), Size { width: 3, height: 2 });
//! assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Empty));
//!
//! grid.set(Pos::new(0, 0), Tile::Wall);
//! assert_eq!(grid.get(Pos::new(0, 0)), Some(&Tile::Wall));
//! ```
mod linear;
pub use linear::GridBuf;

mod view;
pub use view::{GridView, GridViewMut};

pub mod impls;

use crate::{HasSize, TryIntoPos};

/// An error that can occur when creating a grid.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GridError {
    /// The dimensions of the grid are invalid compared to the data provided.
    InvalidDimensions,
}

/// A grid-like structure that allows unchecked read access to its elements.
pub trait GridReadUnchecked {
    /// The type of the elements in the grid.
    type Element;

    /// Returns the element at the given `x` and `y` coordinates.
    ///
    /// If the coordinates are out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `x` and `y` are within the bounds of the grid.
    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element;
}

/// A grid-like structure that can be read from using 2D coordinates.
pub trait GridRead {
    /// The type of the elements in the grid.
    type Element;

    /// Returns a reference to the element at the given position.
    ///
    /// Returns `None` if the position is out of bounds.
    fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&<Self as GridRead>::Element>;
}

/// A grid-like structure that allows unchecked mutable access to its elements.
pub trait GridWriteUnchecked {
    /// The type of the elements in the grid.
    type Element;

    /// Sets the element at the given `x` and `y` coordinates to the specified value.
    ///
    /// If the coordinates are out of bounds, the behavior is undefined.
    ///
    /// # Safety
    ///
    /// The caller must ensure that `x` and `y` are within the bounds of the grid.
    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Element);
}

/// A grid-like structure that can be written to using 2D coordinates.
pub trait GridWrite {
    /// The type of the elements in the grid.
    type Element;

    /// Sets the element at the given position to the specified value.
    fn set(&mut self, pos: impl TryIntoPos<usize>, value: <Self as GridWrite>::Element);
}

/// Extensions on [`GridRead`].
pub trait GridReadExt: GridRead {
    /// Returns a sub-grid view of the grid, defined by the given rectangle.
    ///
    /// If the rectangle is out of bounds, it returns an empty view.
    fn view(&self, rect: impl Into<crate::Rect<usize>>) -> GridView<Self, &Self, Self::Element>
    where
        Self: HasSize<Dim = usize> + Sized,
    {
        let rect = rect.into();
        if rect.right() <= self.size().width && rect.bottom() <= self.size().height {
            unsafe { GridView::new_unchecked(self, rect) }
        } else {
            GridView::empty(self)
        }
    }
}

/// Extensions on [`GridWrite`].
pub trait GridWriteExt: GridWrite {
    /// Returns a mutable sub-grid view of the grid, defined by the given rectangle.
    ///
    /// If the rectangle is out of bounds, it returns an empty view.
    fn view_mut(
        &mut self,
        rect: impl Into<crate::Rect<usize>>,
    ) -> GridViewMut<Self, &mut Self, Self::Element>
    where
        Self: HasSize<Dim = usize> + Sized,
    {
        let rect = rect.into();
        if rect.right() <= self.size().width && rect.bottom() <= self.size().height {
            unsafe { GridViewMut::new_unchecked(self, rect) }
        } else {
            GridViewMut::empty(self)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::{
        HasSize, Pos,
        index::{Layout, RowMajor},
    };

    use super::*;
    use alloc::{vec, vec::Vec};

    struct TestGridUncheckedAndSize {
        data: Vec<i32>,
        width: usize,
    }

    impl GridReadUnchecked for TestGridUncheckedAndSize {
        type Element = i32;

        unsafe fn get_unchecked(&self, x: usize, y: usize) -> &Self::Element {
            &self.data[y * self.width + x]
        }
    }

    impl GridRead for TestGridUncheckedAndSize {
        type Element = i32;

        fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&<Self as GridRead>::Element> {
            unsafe { impls::get_from_unchecked(self, pos) }
        }
    }

    impl GridWriteUnchecked for TestGridUncheckedAndSize {
        type Element = i32;

        unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Element) {
            self.data[y * self.width + x] = value;
        }
    }

    impl GridWrite for TestGridUncheckedAndSize {
        type Element = i32;

        fn set(&mut self, pos: impl TryIntoPos<usize>, value: <Self as GridWrite>::Element) {
            unsafe { impls::set_from_unchecked(self, pos, value) }
        }
    }

    impl HasSize for TestGridUncheckedAndSize {
        type Dim = usize;

        fn size(&self) -> crate::Size<Self::Dim> {
            crate::Size {
                width: self.width,
                height: self.data.len() / self.width,
            }
        }
    }

    #[test]
    fn grid_read_from_unchecked_ok() {
        #[rustfmt::skip]
        let grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        assert_eq!(grid.get(Pos::new(1, 1)), Some(&5));
    }

    #[test]
    fn grid_read_from_unchecked_invalid_pos() {
        #[rustfmt::skip]
        let grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        assert_eq!(grid.get(Pos::new(-1, 1)), None);
        assert_eq!(grid.get(Pos::new(1, -1)), None);
    }

    #[test]
    fn grid_read_from_unchecked_out_of_range_pos() {
        #[rustfmt::skip]
        let grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        assert_eq!(grid.get(Pos::new(3, 1)), None);
        assert_eq!(grid.get(Pos::new(1, 2)), None);
    }

    #[test]
    fn grid_write_from_unchecked_ok() {
        #[rustfmt::skip]
        let mut grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        grid.set(Pos::new(2, 1), 42);
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&42));
    }

    #[test]
    fn grid_write_from_unchecked_invalid_pos() {
        #[rustfmt::skip]
        let mut grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        grid.set(Pos::new(-1, 0), 99);
        grid.set(Pos::new(0, -1), 99);
        // Data should remain unchanged
        assert_eq!(grid.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn grid_write_from_unchecked_out_of_range_pos() {
        #[rustfmt::skip]
        let mut grid = TestGridUncheckedAndSize {
            data: 
            vec![1, 2, 3, 
                 4, 5, 6],
            width: 3,
        };

        grid.set(Pos::new(3, 0), 99);
        grid.set(Pos::new(0, 2), 99);
        // Data should remain unchanged
        assert_eq!(grid.data, vec![1, 2, 3, 4, 5, 6]);
    }

    #[test]
    fn grid_write_and_read_consistency() {
        #[rustfmt::skip]
        let mut grid = TestGridUncheckedAndSize {
            data: 
            vec![0, 0, 0, 
                 0, 0, 0],
            width: 3,
        };

        grid.set(Pos::new(0, 0), 10);
        grid.set(Pos::new(1, 0), 20);
        grid.set(Pos::new(2, 1), 30);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&10));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(&20));
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&30));
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&0));
    }

    struct TestGrid {
        data: Vec<i32>,
        width: usize,
    }

    impl GridRead for TestGrid {
        type Element = i32;

        fn get(&self, pos: impl crate::TryIntoPos<usize>) -> Option<&Self::Element> {
            let pos = pos.try_into_pos().ok()?;
            self.data.get(RowMajor::to_1d(pos, self.width).index)
        }
    }

    impl GridWrite for TestGrid {
        type Element = i32;

        fn set(&mut self, pos: impl crate::TryIntoPos<usize>, value: Self::Element) {
            if let Ok(pos) = pos.try_into_pos() {
                let index = RowMajor::to_1d(pos, self.width).index;
                if index < self.data.len() {
                    self.data[index] = value;
                }
            }
        }
    }

    #[test]
    fn grid_read_write_consistency() {
        let mut grid = TestGrid {
            data: vec![0; 6],
            width: 3,
        };

        grid.set(Pos::new(0, 0), 10);
        grid.set(Pos::new(1, 0), 20);
        grid.set(Pos::new(2, 1), 30);

        assert_eq!(grid.get(Pos::new(0, 0)), Some(&10));
        assert_eq!(grid.get(Pos::new(1, 0)), Some(&20));
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&30));
        assert_eq!(grid.get(Pos::new(1, 1)), Some(&0));
    }
}
