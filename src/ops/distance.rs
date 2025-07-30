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
pub fn euclidean_squared<T: Int>(a: Pos<T>, b: Pos<T>) -> T {
    let dx = a.x - b.x;
    let dy = a.y - b.y;
    dx * dx + dy * dy
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
}
