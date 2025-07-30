//! Line operations.

use crate::{Pos, int::Int};

/// Calculates positions along a line using a fast 2D vector algorithm.
///
/// The resulting iterator is exclusive of the end position; use [`inclusive`][] to include it.
///
/// [`inclusive`]: VectorIter::inclusive
///
/// Considered less "pixel-perfect" than Bresenham's line algorithm, but faster and simpler.
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::line};
///
/// let start = Pos::new(0, 0);
/// let end = Pos::new(2, 2);
/// let mut iter = line::vector(start, end);
/// assert_eq!(iter.next(), Some(Pos::new(0, 0)));
/// assert_eq!(iter.next(), Some(Pos::new(1, 1)));
/// assert_eq!(iter.next(), None);
/// ```
pub fn vector<T: Int>(start: Pos<T>, end: Pos<T>) -> VectorIter<T> {
    let dxy = (end - start).normalized_approx();
    VectorIter {
        pos: start,
        end,
        dxy,
    }
}

pub struct VectorIter<T>
where
    T: Int,
{
    pos: Pos<T>,
    end: Pos<T>,
    dxy: Pos<T>,
}

impl<T> VectorIter<T>
where
    T: Int,
{
    /// Removes a single step from the iterator, making it inclusive of the end position.
    #[must_use]
    pub fn inclusive(self) -> Self {
        Self {
            pos: self.pos,
            end: self.end + self.dxy,
            dxy: self.dxy,
        }
    }
}

impl<T> Iterator for VectorIter<T>
where
    T: Int,
{
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos == self.end {
            None
        } else {
            let current = self.pos;
            self.pos += self.dxy;
            Some(current)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vector_iter_blank() {
        let start = Pos::new(0, 0);
        let end = Pos::new(0, 0);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_horizontal() {
        let start = Pos::new(0, 0);
        let end = Pos::new(5, 0);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(1, 0)));
        assert_eq!(iter.next(), Some(Pos::new(2, 0)));
        assert_eq!(iter.next(), Some(Pos::new(3, 0)));
        assert_eq!(iter.next(), Some(Pos::new(4, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_inclusive() {
        let start = Pos::new(0, 0);
        let end = Pos::new(5, 0);
        let mut iter = vector(start, end).inclusive();
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(1, 0)));
        assert_eq!(iter.next(), Some(Pos::new(2, 0)));
        assert_eq!(iter.next(), Some(Pos::new(3, 0)));
        assert_eq!(iter.next(), Some(Pos::new(4, 0)));
        assert_eq!(iter.next(), Some(Pos::new(5, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_vertical() {
        let start = Pos::new(0, 0);
        let end = Pos::new(0, 5);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(0, 1)));
        assert_eq!(iter.next(), Some(Pos::new(0, 2)));
        assert_eq!(iter.next(), Some(Pos::new(0, 3)));
        assert_eq!(iter.next(), Some(Pos::new(0, 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_diagonal() {
        let start = Pos::new(0, 0);
        let end = Pos::new(3, 3);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(1, 1)));
        assert_eq!(iter.next(), Some(Pos::new(2, 2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_negative() {
        let start = Pos::new(0, 0);
        let end = Pos::new(-3, -3);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(-1, -1)));
        assert_eq!(iter.next(), Some(Pos::new(-2, -2)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_jagged() {
        let start = Pos::new(0, 0);
        let end = Pos::new(6, 4);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(3, 2)));
        assert_eq!(iter.next(), None);
    }
}
