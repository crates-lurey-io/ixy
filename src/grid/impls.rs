//! Reusable bits for implementing grids.

use crate::{
    HasSize, TryIntoPos,
    grid::{GridReadUnchecked, GridWriteUnchecked},
};



/// Implementation of [`GridRead::get`](crate::grid::GridRead::get) for a [`GridReadUnchecked`].
///
/// The size of the grid is referenced from the grid itself.
///
/// # Safety
///
/// This function assumes that [`HasSize::size`] contains valid dimensions for the grid.
pub unsafe fn get_from_unchecked<E>(
    grid: &(impl GridReadUnchecked<Element = E> + HasSize<Dim = usize>),
    pos: impl TryIntoPos<usize>,
) -> Option<&E> {
    let size = grid.size();
    unsafe { get_from_unchecked_with_size(grid, pos, size) }
}

/// Implementation of [`GridRead::get`](crate::grid::GridRead::get) for a [`GridReadUnchecked`]
///
/// The size of the grid is provided as an argument.
///
/// # Safety
///
/// This function assumes that the `size` contains valid dimensions for the grid.
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn get_from_unchecked_with_size<E>(
    grid: &impl GridReadUnchecked<Element = E>,
    pos: impl TryIntoPos<usize>,
    size: impl HasSize<Dim = usize>,
) -> Option<&E> {
    let pos = pos.try_into_pos().ok()?;
    if pos.x >= size.width() || pos.y >= size.height() {
        return None;
    }
    unsafe { Some(grid.get_unchecked(pos.x, pos.y)) }
}

/// Implementation of [`GridWrite::set`](crate::grid::GridWrite::set) for a [`GridWriteUnchecked`].
///
/// The size of the grid is referenced from the grid itself.
///
/// # Safety
///
/// This function assumes that [`HasSize::size`] contains valid dimensions for the grid.
pub unsafe fn set_from_unchecked<E>(
    grid: &mut (impl GridWriteUnchecked<Element = E> + HasSize<Dim = usize>),
    pos: impl TryIntoPos<usize>,
    value: E,
) {
    let size = grid.size();
    unsafe { set_from_unchecked_with_size(grid, pos, value, size) }
}

/// Implementation of [`GridWrite::set`](crate::grid::GridWrite::set) for a [`GridWriteUnchecked`].
///
/// The size of the grid is provided as an argument.
///
/// # Safety
///
/// This function assumes that the `size` contains valid dimensions for the grid.
#[allow(clippy::needless_pass_by_value)]
pub unsafe fn set_from_unchecked_with_size<E>(
    grid: &mut impl GridWriteUnchecked<Element = E>,
    pos: impl TryIntoPos<usize>,
    value: E,
    size: impl HasSize<Dim = usize>,
) {
    let Ok(pos) = pos.try_into_pos() else { return };
    if pos.x >= size.width() || pos.y >= size.height() {
        return;
    }
    unsafe { grid.set_unchecked(pos.x, pos.y, value) }
}
