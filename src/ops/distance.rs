//! Distance operations for positions in a 2D space.

use crate::{Pos, int::Int, internal};

/// Calculates an _approximate_ [Euclidean][] distance between two positions.
///
/// [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_distance
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::distance};
///
/// let a = Pos::new(3, 4);
/// let b = Pos::new(6, 8);
/// assert_eq!(distance::euclidean_approx(a, b), 5);
/// ```
#[must_use]
pub fn euclidean_approx<T: Int>(a: Pos<T>, b: Pos<T>) -> T {
    internal::isqrt(euclidean_squared(a, b))
}

/// Calculates the _squared_ [Euclidean][] distance between two positions.
///
/// [Euclidean]: https://en.wikipedia.org/wiki/Euclidean_distance
///
/// The squared Euclidean distance is the square of the straight-line distance between two points in
/// a 2D space, or `(x₂ - x₁)² + (y₂ - y₁)²`. This is useful for comparing distances without needing
/// to calculate the square root, i.e. when doing a comparison between distances but not using the
/// actual distance value.
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::distance};
///
/// let a = Pos::new(3, 4);
/// let b = Pos::new(6, 8);
/// assert_eq!(distance::euclidean_squared(a, b), 25);
/// ```
#[must_use]
pub fn euclidean_squared<T: Int>(a: Pos<T>, b: Pos<T>) -> T {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
}

/// Calculates the [Manhattan][] (or _taxicab_) distance between two positions.
///
/// [Manhattan]: https://en.wikipedia.org/wiki/Taxicab_geometry
///
/// The Manhattan distance is the sum of the absolute differences of the coordinates, or
/// `|x₂ - x₁| + |y₂ - y₁|`. This is the natural distance metric for grids that only allow
/// 4-directional (cardinal) movement.
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::distance};
///
/// let a = Pos::new(1, 1);
/// let b = Pos::new(4, 5);
/// assert_eq!(distance::manhattan(a, b), 7);
/// ```
#[must_use]
pub fn manhattan<T: Int>(a: Pos<T>, b: Pos<T>) -> T {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

/// Calculates the [Chebyshev][] (or _chessboard_) distance between two positions.
///
/// [Chebyshev]: https://en.wikipedia.org/wiki/Chebyshev_distance
///
/// The Chebyshev distance is the greater of the absolute differences of the coordinates, or
/// `max(|x₂ - x₁|, |y₂ - y₁|)`. This is the natural distance metric for grids that allow
/// 8-directional (cardinal and diagonal) movement, such as a king's moves in chess.
///
/// ## Examples
///
/// ```rust
/// use ixy::{Pos, ops::distance};
///
/// let a = Pos::new(1, 1);
/// let b = Pos::new(4, 5);
/// assert_eq!(distance::chebyshev(a, b), 4);
/// ```
#[must_use]
pub fn chebyshev<T: Int>(a: Pos<T>, b: Pos<T>) -> T {
    let dx = (a.x - b.x).abs();
    let dy = (a.y - b.y).abs();
    if dx > dy { dx } else { dy }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn euclidean_approx_test() {
        let a = Pos::new(3, 4);
        let b = Pos::new(6, 8);
        assert_eq!(euclidean_approx(a, b), 5);
    }

    #[test]
    fn euclidean_squared_test() {
        let a = Pos::new(3, 4);
        let b = Pos::new(6, 8);
        assert_eq!(euclidean_squared(a, b), 25);
    }

    #[test]
    fn manhattan_test() {
        let a = Pos::new(1, 1);
        let b = Pos::new(4, 5);
        assert_eq!(manhattan(a, b), 7);
    }

    #[test]
    fn manhattan_symmetric() {
        let a = Pos::new(4, 5);
        let b = Pos::new(1, 1);
        assert_eq!(manhattan(a, b), manhattan(b, a));
    }

    #[test]
    fn manhattan_zero() {
        let a = Pos::new(3, 3);
        assert_eq!(manhattan(a, a), 0);
    }

    #[test]
    fn chebyshev_test() {
        let a = Pos::new(1, 1);
        let b = Pos::new(4, 5);
        assert_eq!(chebyshev(a, b), 4);
    }

    #[test]
    fn chebyshev_axis_aligned() {
        let a = Pos::new(0, 0);
        let b = Pos::new(5, 0);
        assert_eq!(chebyshev(a, b), 5);
    }

    #[test]
    fn chebyshev_zero() {
        let a = Pos::new(3, 3);
        assert_eq!(chebyshev(a, a), 0);
    }
}
