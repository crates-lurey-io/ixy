//! General purpose iterators for accessing elements in a grid.

use crate::grid::{GridRead, GridReadUnchecked};
use crate::{Pos, TryIntoPos};

/// Yields the elements of a `GridRead` from the positions yielded by the given iterator.
///
/// # Examples
///
/// ```rust
/// use ixy::{
///    grid::{iter::iter_from_pos, GridRead, GridBuf},
///    Pos,
/// };
///
/// let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
/// let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
/// let elements: Vec<_> = iter_from_pos(&grid, positions.into_iter()).collect();
///
/// assert_eq!(elements, &[&1, &5, &6]);
/// ```
pub fn iter_from_pos<'a, E: 'a>(
    grid: &'a impl GridRead<Element = E>,
    pos: impl Iterator<Item = impl TryIntoPos<usize>>,
) -> impl Iterator<Item = &'a E> {
    pos.filter_map(move |pos| grid.get(pos))
}

/// Yields the position and element from the positions yielded by the given iterator.
///
/// # Examples
///
/// ```rust
/// use ixy::{
///   grid::{iter::iter_with_pos, GridRead, GridBuf},
///   Pos,
/// };
///
/// let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
/// let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
/// let elements: Vec<_> = iter_with_pos(&grid, positions.into_iter()).collect();
///
/// assert_eq!(elements, &[(Pos::new(0, 0), &1), (Pos::new(1, 1), &5), (Pos::new(2, 1), &6)]);
/// ```
pub fn iter_with_pos<'a, E: 'a>(
    grid: &'a impl GridRead<Element = E>,
    pos: impl Iterator<Item = impl TryIntoPos<usize>>,
) -> impl Iterator<Item = (Pos<usize>, &'a E)> {
    pos.filter_map(move |pos| {
        let pos = pos.try_into_pos().ok()?;
        grid.get(pos).map(|elem| (pos, elem))
    })
}

/// Yields the elements of a `GridReadUnchecked` from the positions yielded by the given iterator.
///
/// # Safety
///
/// This function assumes that the positions yielded by the iterator are valid for the grid.
///
/// # Examples
///
/// ```rust
/// use ixy::{
///    grid::{iter::iter_from_pos_unchecked, GridReadUnchecked, GridBuf},
///    Pos,
/// };
///
/// let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
/// let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
/// let elements: Vec<_> = unsafe { iter_from_pos_unchecked(&grid, positions.into_iter()) }.collect();
///
/// assert_eq!(elements, &[&1, &5, &6]);
/// ```
pub unsafe fn iter_from_pos_unchecked<'a, E: 'a>(
    grid: &'a impl GridReadUnchecked<Element = E>,
    pos: impl Iterator<Item = impl TryIntoPos<usize>>,
) -> impl Iterator<Item = &'a E> {
    pos.filter_map(move |pos| {
        let pos = pos.try_into_pos().ok()?;
        Some(unsafe { grid.get_unchecked(pos.x, pos.y) })
    })
}

/// Yields the position and element from the positions yielded by the given iterator.
///
/// # Safety
///
/// This function assumes that the positions yielded by the iterator are valid for the grid.
///
/// # Examples
///
/// ```rust
/// use ixy::{
///    grid::{iter::iter_with_pos_unchecked, GridReadUnchecked, GridBuf},
///    Pos,
/// };
///
/// let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
/// let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
/// let elements: Vec<_> = unsafe { iter_with_pos_unchecked(&grid, positions.into_iter()) }.collect();
///
/// assert_eq!(
///     elements,
///     &[(Pos::new(0, 0), &1), (Pos::new(1, 1), &5), (Pos::new(2, 1), &6)]
/// );
/// ```
pub unsafe fn iter_with_pos_unchecked<'a, E: 'a>(
    grid: &'a impl GridReadUnchecked<Element = E>,
    pos: impl Iterator<Item = impl TryIntoPos<usize>>,
) -> impl Iterator<Item = (Pos<usize>, &'a E)> {
    pos.filter_map(move |pos| {
        let pos = pos.try_into_pos().ok()?;
        Some((pos, unsafe { grid.get_unchecked(pos.x, pos.y) }))
    })
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::grid::GridBuf;
    use alloc::{vec, vec::Vec};

    #[test]
    fn test_iter_from_pos_basic() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
        let elements: Vec<_> = iter_from_pos(&grid, positions.into_iter()).collect();
        assert_eq!(elements, &[&1, &5, &6]);
    }

    #[test]
    fn test_iter_from_pos_out_of_bounds() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![
            Pos::new(0, 0),
            Pos::new(3, 0), // out of bounds
            Pos::new(1, 1),
            Pos::new(2, 2), // out of bounds
        ];
        let elements: Vec<_> = iter_from_pos(&grid, positions.into_iter()).collect();
        assert_eq!(elements, &[&1, &5]);
    }

    #[test]
    fn test_iter_with_pos_basic() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
        let elements: Vec<_> = iter_with_pos(&grid, positions.into_iter()).collect();
        assert_eq!(
            elements,
            &[
                (Pos::new(0, 0), &1),
                (Pos::new(1, 1), &5),
                (Pos::new(2, 1), &6)
            ]
        );
    }

    #[test]
    fn test_iter_with_pos_out_of_bounds() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![
            Pos::new(0, 0),
            Pos::new(3, 0), // out of bounds
            Pos::new(1, 1),
            Pos::new(2, 2), // out of bounds
        ];
        let elements: Vec<_> = iter_with_pos(&grid, positions.into_iter()).collect();
        assert_eq!(elements, &[(Pos::new(0, 0), &1), (Pos::new(1, 1), &5)]);
    }

    #[test]
    fn test_iter_from_pos_unchecked_basic() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
        let elements: Vec<_> =
            unsafe { iter_from_pos_unchecked(&grid, positions.into_iter()) }.collect();
        assert_eq!(elements, &[&1, &5, &6]);
    }

    #[test]
    fn test_iter_with_pos_unchecked_basic() {
        let grid = GridBuf::from_row_major(3, 2, vec![1, 2, 3, 4, 5, 6]).unwrap();
        let positions = vec![Pos::new(0, 0), Pos::new(1, 1), Pos::new(2, 1)];
        let elements: Vec<_> =
            unsafe { iter_with_pos_unchecked(&grid, positions.into_iter()) }.collect();
        assert_eq!(
            elements,
            &[
                (Pos::new(0, 0), &1),
                (Pos::new(1, 1), &5),
                (Pos::new(2, 1), &6)
            ]
        );
    }

    #[test]
    fn test_iter_from_pos_empty_positions() {
        let grid = GridBuf::from_row_major(2, 2, vec![1, 2, 3, 4]).unwrap();
        let positions: Vec<Pos<usize>> = vec![];
        let elements: Vec<_> = iter_from_pos(&grid, positions.into_iter()).collect();
        assert!(elements.is_empty());
    }

    #[test]
    fn test_iter_with_pos_empty_positions() {
        let grid = GridBuf::from_row_major(2, 2, vec![1, 2, 3, 4]).unwrap();
        let positions: Vec<Pos<usize>> = vec![];
        let elements: Vec<_> = iter_with_pos(&grid, positions.into_iter()).collect();
        assert!(elements.is_empty());
    }
}
