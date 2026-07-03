//! Line operations.

use core::iter::FusedIterator;

use crate::{
    Pos,
    int::{Int, SignedInt},
    ops::distance,
};

/// Calculates positions along a line using a fast 2D vector algorithm.
///
/// The resulting iterator is _inclusive_ of the end position.
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
/// assert_eq!(iter.next(), Some(Pos::new(2, 2)));
/// assert_eq!(iter.next(), None);
/// ```
pub fn vector<T: Int>(start: Pos<T>, end: Pos<T>) -> impl Iterator<Item = Pos<T>> {
    let dxy = (end - start).normalized_approx();
    VectorIter {
        pos: start,
        end: end + dxy,
        dxy,
    }
}

struct VectorIter<T>
where
    T: Int,
{
    pos: Pos<T>,
    end: Pos<T>,
    dxy: Pos<T>,
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T> ExactSizeIterator for VectorIter<T>
where
    T: Int,
{
    fn len(&self) -> usize {
        if self.pos == self.end {
            0
        } else {
            let remaining = self.end - self.pos;
            remaining.x.abs().max(remaining.y.abs()).to_usize()
        }
    }
}

impl<T> FusedIterator for VectorIter<T> where T: Int {}

/// Calculates positions along a line using [Bresenham's line algorithm][bresenham].
///
/// [bresenham]: https://en.wikipedia.org/wiki/Bresenham%27s_line_algorithm
///
/// The resulting iterator is _inclusive_ of the end position, and always yields exactly
/// `max(|dx|, |dy|) + 1` positions, where `dx` and `dy` are the coordinate deltas between
/// `start` and `end`.
///
/// Unlike [`vector`], this produces the pixel-perfect line a rasterizer would draw, at the cost
/// of slightly more per-step bookkeeping.
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::line};
///
/// let start = Pos::new(0, 0);
/// let end = Pos::new(6, 4);
/// let positions: Vec<_> = line::bresenham(start, end).collect();
/// assert_eq!(
///     positions,
///     &[
///         Pos::new(0, 0),
///         Pos::new(1, 1),
///         Pos::new(2, 1),
///         Pos::new(3, 2),
///         Pos::new(4, 3),
///         Pos::new(5, 3),
///         Pos::new(6, 4),
///     ]
/// );
/// ```
pub fn bresenham<T: SignedInt>(start: Pos<T>, end: Pos<T>) -> impl Iterator<Item = Pos<T>> {
    let dx = (end.x - start.x).abs();
    let dy = -(end.y - start.y).abs();
    let sx = if start.x < end.x { T::ONE } else { T::NEG_ONE };
    let sy = if start.y < end.y { T::ONE } else { T::NEG_ONE };
    BresenhamIter {
        pos: start,
        end,
        dx,
        dy,
        sx,
        sy,
        err: dx + dy,
        done: false,
    }
}

struct BresenhamIter<T: SignedInt> {
    pos: Pos<T>,
    end: Pos<T>,
    dx: T,
    dy: T,
    sx: T,
    sy: T,
    err: T,
    done: bool,
}

impl<T: SignedInt> Iterator for BresenhamIter<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.done {
            return None;
        }
        let current = self.pos;
        if self.pos == self.end {
            self.done = true;
            return Some(current);
        }
        let e2 = self.err + self.err;
        if e2 >= self.dy {
            self.err += self.dy;
            self.pos.x += self.sx;
        }
        if e2 <= self.dx {
            self.err += self.dx;
            self.pos.y += self.sy;
        }
        Some(current)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: SignedInt> ExactSizeIterator for BresenhamIter<T> {
    fn len(&self) -> usize {
        if self.done {
            0
        } else {
            distance::chebyshev(self.pos, self.end).to_usize() + 1
        }
    }
}

impl<T: SignedInt> FusedIterator for BresenhamIter<T> {}

#[cfg(test)]
mod tests {
    extern crate alloc;

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
        assert_eq!(iter.next(), Some(Pos::new(0, 5)));
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
        assert_eq!(iter.next(), Some(Pos::new(3, 3)));
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
        assert_eq!(iter.next(), Some(Pos::new(-3, -3)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_jagged() {
        let start = Pos::new(0, 0);
        let end = Pos::new(6, 4);
        let mut iter = vector(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), Some(Pos::new(3, 2)));
        assert_eq!(iter.next(), Some(Pos::new(6, 4)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn vector_iter_size_hint() {
        let start = Pos::new(0, 0);
        let end = Pos::new(3, 3);
        let mut iter = vector(start, end);
        assert_eq!(iter.size_hint(), (4, Some(4)));
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.size_hint(), (3, Some(3)));
        assert_eq!(iter.next(), Some(Pos::new(1, 1)));
        assert_eq!(iter.size_hint(), (2, Some(2)));
        assert_eq!(iter.next(), Some(Pos::new(2, 2)));
        assert_eq!(iter.size_hint(), (1, Some(1)));
        assert_eq!(iter.next(), Some(Pos::new(3, 3)));
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn bresenham_degenerate() {
        let start = Pos::new(0, 0);
        let end = Pos::new(0, 0);
        let mut iter = bresenham(start, end);
        assert_eq!(iter.next(), Some(Pos::new(0, 0)));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn bresenham_horizontal() {
        let start = Pos::new(0, 0);
        let end = Pos::new(5, 0);
        let positions: alloc::vec::Vec<_> = bresenham(start, end).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(3, 0),
                Pos::new(4, 0),
                Pos::new(5, 0),
            ]
        );
    }

    #[test]
    fn bresenham_vertical() {
        let start = Pos::new(0, 0);
        let end = Pos::new(0, 5);
        let positions: alloc::vec::Vec<_> = bresenham(start, end).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(0, 2),
                Pos::new(0, 3),
                Pos::new(0, 4),
                Pos::new(0, 5),
            ]
        );
    }

    #[test]
    fn bresenham_diagonal() {
        let start = Pos::new(0, 0);
        let end = Pos::new(3, 3);
        let positions: alloc::vec::Vec<_> = bresenham(start, end).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(1, 1),
                Pos::new(2, 2),
                Pos::new(3, 3),
            ]
        );
    }

    #[test]
    fn bresenham_shallow_slope_is_pixel_perfect() {
        // Unlike `vector`, bresenham never skips intermediate cells.
        let start = Pos::new(0, 0);
        let end = Pos::new(6, 4);
        let positions: alloc::vec::Vec<_> = bresenham(start, end).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(1, 1),
                Pos::new(2, 1),
                Pos::new(3, 2),
                Pos::new(4, 3),
                Pos::new(5, 3),
                Pos::new(6, 4),
            ]
        );
    }

    #[test]
    fn bresenham_negative_direction() {
        let start = Pos::new(0, 0);
        let end = Pos::new(-3, -3);
        let positions: alloc::vec::Vec<_> = bresenham(start, end).collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(-1, -1),
                Pos::new(-2, -2),
                Pos::new(-3, -3),
            ]
        );
    }

    #[test]
    fn bresenham_len_matches_chebyshev_plus_one() {
        let start = Pos::new(0, 0);
        let end = Pos::new(6, 4);
        let mut iter = bresenham(start, end);
        assert_eq!(iter.size_hint(), (7, Some(7)));
        iter.next();
        assert_eq!(iter.size_hint(), (6, Some(6)));
        assert_eq!(iter.count(), 6);
    }
}
