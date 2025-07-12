use core::{
    marker::PhantomData,
    ops::{Deref},
};

use crate::{
    HasSize, Rect,
    grid::{
        GridError, GridRead, GridReadUnchecked,
    },
};

/// A grid that provides read-only access as a view of a larger grid.
///
/// Grid views are useful for creating sub-regions of a grid without copying the data.
///
/// See also: [`GridReadExt::view`](crate::grid::GridReadExt::view).
///
/// # Examples
///
/// ```rust
/// use ixy::{HasSize, Pos, Rect, Size, grid::{GridBuf, GridRead, GridView}};
///
/// #[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// enum Tile {
///   Empty,
///   Wall,
/// }
///
/// let mut cells = vec![Tile::Empty; 9];
/// cells[8] = Tile::Wall;
///
/// let grid = GridBuf::from_row_major(3, 3, cells).unwrap();
/// let view = GridView::<_, _, Tile>::new(&grid, Rect::from_ltwh_unsigned(1, 1, 2, 2)).unwrap();
///
/// assert_eq!(view.size(), Size { width: 2, height: 2 });
/// assert_eq!(view.get(Pos::new(0, 0)), Some(&Tile::Empty));
/// assert_eq!(view.get(Pos::new(1, 1)), Some(&Tile::Wall));
/// ```
pub struct GridView<T, G: Deref<Target = T>, E> {
    grid: G,
    rect: Rect<usize>,
    cell: PhantomData<E>,
}

impl<T, G: Deref<Target = T>, E> GridView<T, G, E> {
    /// Creates an empty `GridView` (e.g. a `0x0` region).
    pub const fn empty(grid: G) -> Self {
        Self {
            grid,
            rect: Rect::EMPTY,
            cell: PhantomData,
        }
    }

    /// Creates a new `GridView` with the specified sub-bounds defined by `rect`.
    ///
    /// Methods to the grid are delegated to the underlying grid, with the following adjustments:
    /// - The `x` coordinate is offset by the left edge of the `rect`;
    /// - The `y` coordinate is offset by the top edge of the `rect`;
    /// - The maximum `x` coordinate is adjusted to the right edge of the `rect`;
    /// - The maximum `y` coordinate is adjusted to the bottom edge of the `rect`.
    ///
    /// # Errors
    ///
    /// If the `rect` is out of bounds of the grid, an error is returned.
    pub fn new(grid: G, rect: Rect<usize>) -> Result<Self, GridError>
    where
        T: HasSize<Dim = usize>,
    {
        let size = grid.size();
        if rect.right() <= size.width && rect.bottom() <= size.height {
            unsafe { Ok(Self::new_unchecked(grid, rect)) }
        } else {
            Err(GridError::InvalidDimensions)
        }
    }

    /// Creates a new `GridView` with the specified sub-bounds defined by `rect`.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `rect` is within the bounds of the grid.
    pub unsafe fn new_unchecked(grid: G, rect: Rect<usize>) -> Self {
        Self {
            grid,
            rect,
            cell: PhantomData,
        }
    }
}

impl<T, G: Deref<Target = T>, E> HasSize for GridView<T, G, E>
where
    T: HasSize,
{
    type Dim = usize;

    fn size(&self) -> crate::Size<Self::Dim> {
        self.rect.size()
    }
}

impl<T, G: Deref<Target = T>, E> GridReadUnchecked for GridView<T, G, E>
where
    T: GridReadUnchecked<Element = E>,
{
    type Element = E;

    unsafe fn get_unchecked(&self, x: usize, y: usize) -> &E {
        let x = self.rect.left() + x;
        let y = self.rect.top() + y;
        unsafe { self.grid.get_unchecked(x, y) }
    }
}

impl<T, G: Deref<Target = T>, E> GridRead for GridView<T, G, E>
where
    T: GridRead<Element = E>,
{
    type Element = E;

    fn get(&self, pos: impl crate::TryIntoPos<usize>) -> Option<&<Self as GridRead>::Element> {
        let pos = pos.try_into_pos().ok()?;
        self.grid.get(pos + self.rect.top_left())
    }
}

/// Extensions on a grid that provide methods for creating sub-views.
pub trait GridSubView {
    /// The element type of the grid.
    type Element;

    /// Returns a sub-grid view of the grid, defined by the given rectangle.
    ///
    /// # Safety
    ///
    /// The caller must ensure that the `rect` is within the bounds of the grid.
    unsafe fn view_unchecked(&self, rect: Rect<usize>) -> GridView<Self, &Self, Self::Element>
    where
        Self: Sized,
    {
        unsafe { GridView::new_unchecked(self, rect) }
    }

    /// Returns a sub-grid view of the grid, defined by the given rectangle.
    ///
    /// # Errors
    ///
    /// If the `rect` is out of bounds of the grid, an error is returned.
    fn view(&self, rect: Rect<usize>) -> Result<GridView<Self, &Self, Self::Element>, GridError>
    where
        Self: Sized;
}

impl<T: GridRead + HasSize<Dim = usize>> GridSubView for T {
    type Element = <T as GridRead>::Element;

    fn view(&self, rect: Rect<usize>) -> Result<GridView<Self, &Self, Self::Element>, GridError>
    where
        Self: Sized,
    {
        GridView::new(self, rect)
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::{
        Pos,
        grid::{GridBuf, GridReadMut},
    };
    use alloc::vec;

    #[test]
    fn view_2x2_of_3x3() {
        #[rustfmt::skip]
        let mut grid = GridBuf::from_row_major(3, 3, vec![
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9,
        ]).unwrap();

        let rect = Rect::from_ltwh_unsigned(1, 1, 2, 2);
        let mut view = GridView::new(&grid, rect).unwrap();

        assert_eq!(
            view.size(),
            crate::Size {
                width: 2,
                height: 2
            }
        );

        assert_eq!(view.get(Pos::new(0, 0)), Some(&5));
        assert_eq!(view.get(Pos::new(1, 0)), Some(&6));
        assert_eq!(view.get(Pos::new(0, 1)), Some(&8));
        assert_eq!(view.get(Pos::new(1, 1)), Some(&9));
        assert_eq!(view.get(Pos::new(2, 2)), None); // Out

        // Test get_unchecked for GridView
        unsafe {
            assert_eq!(*view.get_unchecked(0, 0), 5);
            assert_eq!(*view.get_unchecked(1, 0), 6);
            assert_eq!(*view.get_unchecked(0, 1), 8);
            assert_eq!(*view.get_unchecked(1, 1), 9);
        }

        // Test get_mut for GridViewMut
        *view.get_mut(Pos::new(1, 1)).unwrap() = 10;
        assert_eq!(view.get(Pos::new(0, 0)), Some(&10));
    }

    #[test]
    fn view_2x2_of_3x3_mut() {
        #[rustfmt::skip]
        let mut grid = GridBuf::from_row_major(3, 3, vec![
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9,
        ]).unwrap();

        let rect = Rect::from_ltwh_unsigned(1, 1, 2, 2);
        let mut view = GridViewMut::new(&mut grid, rect).unwrap();

        assert_eq!(
            view.size(),
            crate::Size {
                width: 2,
                height: 2
            }
        );

        view.set(Pos::new(0, 0), 10);
        view.set(Pos::new(1, 0), 11);
        view.set(Pos::new(0, 1), 12);
        view.set(Pos::new(1, 1), 13);

        assert_eq!(grid.get(Pos::new(1, 1)), Some(&10));
        assert_eq!(grid.get(Pos::new(2, 1)), Some(&11));
        assert_eq!(grid.get(Pos::new(1, 2)), Some(&12));
        assert_eq!(grid.get(Pos::new(2, 2)), Some(&13));
    }

    #[test]
    fn empty() {
        let grid = GridBuf::from_row_major(1, 1, vec![1]).unwrap();
        let view = GridView::<_, _, i32>::empty(&grid);
        assert_eq!(
            view.size(),
            crate::Size {
                width: 0,
                height: 0
            }
        );
    }
}
