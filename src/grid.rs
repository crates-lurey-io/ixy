mod linear;
pub use linear::LinearGridBuf;

use crate::{HasSize, TryIntoPos};

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
    fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&Self::Element>;
}

impl<G: GridReadUnchecked + HasSize<Dim = usize>> GridRead for G {
    type Element = G::Element;

    fn get(&self, pos: impl TryIntoPos<usize>) -> Option<&Self::Element> {
        let (x, y) = *pos.try_into_pos().ok()?.as_ref();
        if x >= self.width() || y >= self.height() {
            return None;
        }
        Some(unsafe { self.get_unchecked(x, y) })
    }
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
    fn set(&mut self, pos: impl TryIntoPos<usize>, value: Self::Element);
}

impl<Grid: GridWriteUnchecked + HasSize<Dim = usize>> GridWrite for Grid {
    type Element = Grid::Element;

    fn set(&mut self, pos: impl TryIntoPos<usize>, value: Self::Element) {
        if let Ok(pos) = pos.try_into_pos() {
            let (x, y) = *pos.as_ref();
            if x >= self.width() || y >= self.height() {
                return; // Out of bounds, do nothing
            }
            unsafe { self.set_unchecked(x, y, value) };
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use crate::Pos;

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

    impl GridWriteUnchecked for TestGridUncheckedAndSize {
        type Element = i32;

        unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: Self::Element) {
            self.data[y * self.width + x] = value;
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
}
