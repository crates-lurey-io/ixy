use core::{
    marker::PhantomData,
    ops::{Deref, DerefMut},
};

use crate::{
    HasSize, Rect,
    grid::{GridError, GridRead, GridReadUnchecked, GridWrite, GridWriteUnchecked},
};

/// A grid that provides read-only access as a view of a larger grid.
pub struct GridView<T, G: Deref<Target = T>, E> {
    grid: G,
    rect: Rect<usize>,
    cell: PhantomData<E>,
}

impl<T, G: Deref<Target = T>, E> GridView<T, G, E> {
    /// Creates a new `GridView` with the specified sub-bounds defined by `rect`.
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

pub struct GridViewMut<T, G: DerefMut<Target = T>, E> {
    grid: G,
    rect: Rect<usize>,
    cell: PhantomData<E>,
}

impl<T, G: DerefMut<Target = T>, E> GridViewMut<T, G, E> {
    /// Creates a new `GridViewMut` with the specified sub-bounds defined by `rect`.
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

    /// Creates a new `GridViewMut` with the specified sub-bounds defined by `rect`.
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

impl<T, G: DerefMut<Target = T>, E> HasSize for GridViewMut<T, G, E>
where
    T: HasSize,
{
    type Dim = usize;

    fn size(&self) -> crate::Size<Self::Dim> {
        self.rect.size()
    }
}

impl<T, G: DerefMut<Target = T>, E> GridReadUnchecked for GridViewMut<T, G, E>
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

impl<T, G: DerefMut<Target = T>, E> GridRead for GridViewMut<T, G, E>
where
    T: GridRead<Element = E>,
{
    type Element = E;

    fn get(&self, pos: impl crate::TryIntoPos<usize>) -> Option<&<Self as GridRead>::Element> {
        let pos = pos.try_into_pos().ok()?;
        self.grid.get(pos + self.rect.top_left())
    }
}

impl<T, G: DerefMut<Target = T>, E> GridWriteUnchecked for GridViewMut<T, G, E>
where
    T: GridWriteUnchecked<Element = E>,
{
    type Element = E;

    unsafe fn set_unchecked(&mut self, x: usize, y: usize, value: E) {
        let x = self.rect.left() + x;
        let y = self.rect.top() + y;
        unsafe { self.grid.set_unchecked(x, y, value) }
    }
}

impl<T, G: DerefMut<Target = T>, E> GridWrite for GridViewMut<T, G, E>
where
    T: GridWrite<Element = E>,
{
    type Element = E;

    fn set(&mut self, pos: impl crate::TryIntoPos<usize>, value: E) {
        let Ok(pos) = pos.try_into_pos() else {
            return;
        };
        self.grid.set(pos + self.rect.top_left(), value);
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::{Pos, grid::GridBuf};
    use alloc::vec;

    #[test]
    fn view_2x2_of_3x3() {
        #[rustfmt::skip]
        let grid = GridBuf::from_row_major(3, 3, vec![
            1, 2, 3, 
            4, 5, 6, 
            7, 8, 9,
        ]).unwrap();

        let rect = Rect::from_ltwh_unsigned(1, 1, 2, 2);
        let view = GridView::new(&grid, rect).unwrap();

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
}
