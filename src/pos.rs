use core::{fmt::Display, ops};

use crate::{
    Rect,
    int::{Int, SignedInt},
    internal,
};

/// Applies an unsigned `magnitude` to `base` in the given direction, saturating at the bounds of
/// `T` instead of overflowing or underflowing.
///
/// The magnitude is applied in chunks of at most `T::MAX`, and the loop exits as soon as `base`
/// has saturated at the relevant bound, so this terminates in a small, constant number of
/// iterations (at most a handful) regardless of how large `magnitude` is or how narrow `T` is.
/// This avoids the pitfall of a single `T::saturating_from_usize(magnitude)` step, which would
/// clamp the magnitude itself before it is applied and under-apply large deltas to narrow signed
/// types (whose `MIN` magnitude is one greater than `MAX`).
fn saturating_apply_magnitude<T: Int>(base: T, magnitude: usize, negative: bool) -> T {
    // `saturating_to_usize`, not `to_usize`: `T::MAX` genuinely exceeds `usize` for `u128`/`i128`
    // on every supported target, and a `usize::MAX` chunk is the right cap there anyway.
    let cap = T::MAX.saturating_to_usize();
    let bound = if negative { T::MIN } else { T::MAX };
    let mut result = base;
    let mut remaining = magnitude;
    while remaining > 0 && result != bound && cap > 0 {
        let chunk = remaining.min(cap);
        let step = T::saturating_from_usize(chunk);
        result = if negative {
            result.saturating_sub(step)
        } else {
            result.saturating_add(step)
        };
        remaining -= chunk;
    }
    result
}

/// The magnitude of `delta` as a `usize`, saturating on targets whose `usize` is narrower.
fn to_magnitude(delta: i32) -> usize {
    usize::try_from(delta.unsigned_abs()).unwrap_or(usize::MAX)
}

/// A macro that creates a position with the given `x` and `y` coordinates.
#[macro_export]
macro_rules! pos {
    ($x:expr, $y:expr) => {
        Pos::new($x, $y)
    };
}

/// A 2-dimensional point with integer precision.
///
/// The type parameter `T` is guaranteed to be a built-in Rust integer type, and defaults to `i32`.
///
/// ## Layout
///
/// The layout of `Pos<T>` is guaranteed to be the same as a C struct with two fields, `x` and `y`,
/// both of type `T`.
///
/// For example, a `Pos<i32>` is equivalent to the following C struct:
///
/// ```c
/// struct Pos {
///   int x;
///   int y;
/// }
/// ```
///
/// ## Ordering
///
/// Points are ordered _row-major_, or the point with the smaller `y` coordinate comes first.
///
/// If two points have the same `y` coordinate, the point with the smaller `x` coordinate is first.
///
/// This is the natural ordering for grid iteration: top-to-bottom, left-to-right within each row.
/// For lexicographic (x-primary) ordering, use [`Pos::cmp_lexicographic`].
///
/// ```rust
/// use ixy::Pos;
///
/// assert!(Pos::new(0, 3) > Pos::new(1, 2));   // y-primary: 3 > 2
/// assert!(Pos::new(1, 2) > Pos::new(2, 1));   // y-primary: 2 > 1
/// ```
///
/// ## Examples
///
/// Create a point, also known as a position, at `(3, 4)`:
///
/// ```rust
/// use ixy::Pos;
///
/// let p = Pos::new(3, 4);
/// assert_eq!(p.x, 3);
/// assert_eq!(p.y, 4);
/// ```
///
/// Or, to use a specific integer type, such as `u16`:
///
/// ```rust
/// use ixy::Pos;
///
/// let p = Pos::<u16>::new(3, 4);
/// assert_eq!(p.x, 3);
/// assert_eq!(p.y, 4);
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(C)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pos<T = i32> {
    /// The x-coordinate, or _horizontal_ position from the origin.
    ///
    /// ```txt
    /// (x increases →)
    /// +---------→ x
    /// |
    /// |
    /// ↓
    /// y
    /// ```
    pub x: T,

    /// The y-coordinate, or _vertical_ position from the origin.
    ///
    /// ```txt
    /// (y increases ↓)
    /// +---------→ x
    /// |
    /// |
    /// ↓
    /// y
    /// ```
    pub y: T,
}

#[allow(private_bounds)]
impl<T: Int> Pos<T> {
    /// Origin point, i.e. `(0, 0)`.
    ///
    /// This is the same value returned by [`Pos::default()`].
    ///
    /// ```txt
    ///      ↑
    ///      |
    ///      |
    /// ←----O----→
    ///      |
    ///      |
    ///      ↓
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::ORIGIN, Pos::new(0, 0));
    /// ```
    pub const ORIGIN: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
    };

    /// The zero vector, i.e. `(0, 0)`.
    ///
    /// An alias for [`Pos::ORIGIN`], provided for parity with other geometry crates that use
    /// `ZERO` for the additive identity of a vector type.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::ZERO, Pos::new(0, 0));
    /// ```
    pub const ZERO: Self = Self::ORIGIN;

    /// The minimum point, i.e. `(T::MIN, T::MIN)`.
    ///
    /// For unsigned integers, this is always [`Self::ORIGIN`], or `O` in the diagram below:
    ///
    /// ```txt
    /// O---------→ x
    /// |
    /// |
    /// ↓
    /// y
    /// ```
    ///
    /// For signed integers, this is the negation of [`Self::MAX`], or `P` in the diagram below:
    /// ```txt
    /// P    ↑
    ///      |
    ///      |
    /// ←----O----→
    ///      |
    ///      |
    ///      ↓
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::<i32>::MIN, Pos::<i32>::new(-2147483648, -2147483648));
    /// assert_eq!(Pos::<u32>::MIN, Pos::<u32>::new(0, 0));
    /// ```
    pub const MIN: Self = Self {
        x: T::MIN,
        y: T::MIN,
    };

    /// The maximum point, i.e. `(T::MAX, T::MAX)`.
    ///
    /// Each value is the maximum value of the integer type `T` (i.e. `i32::MAX` for `Pos<i32>`).
    ///
    /// ```txt
    /// O---------→ x
    /// |
    /// |
    /// ↓
    /// y           P
    /// ```
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::<i32>::MAX, Pos::<i32>::new(2147483647, 2147483647));
    /// assert_eq!(Pos::<u32>::MAX, Pos::<u32>::new(4294967295, 4294967295));
    /// ```
    pub const MAX: Self = Self {
        x: T::MAX,
        y: T::MAX,
    };

    /// A unit vector of length `1` in the positive x-direction, i.e. `(1, 0)`.
    ///
    /// Useful in combination with [`ops::Mul`] or [`ops::MulAssign`] to scale the vector.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::X * 5; // Scales the unit vector by 5
    /// assert_eq!(p, Pos::new(5, 0));
    ///
    /// let mut q = Pos::X;
    /// q *= 3; // Scales the unit vector by 3
    /// assert_eq!(q, Pos::new(3, 0));
    /// ```
    pub const X: Self = Self {
        x: T::ONE,
        y: T::ZERO,
    };

    /// A unit vector of length `1` in the positive y-direction, i.e. `(0, 1)`.
    ///
    /// Useful in combination with [`ops::Mul`] or [`ops::MulAssign`] to scale the vector.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::Y * 5; // Scales the unit vector by 5
    /// assert_eq!(p, Pos::new(0, 5));
    ///
    /// let mut q = Pos::Y;
    /// q *= 3; // Scales the unit vector by 3
    /// assert_eq!(q, Pos::new(0, 3));
    /// ```
    pub const Y: Self = Self {
        x: T::ZERO,
        y: T::ONE,
    };

    /// Creates a new point with the given `x` and `y` coordinates.
    ///
    /// An alternative to using the `Pos { x, y }` syntax.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::new(3, 4), Pos { x: 3, y: 4 });
    /// ```
    #[must_use]
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// Returns an approximate normalized vector of the position.
    ///
    /// Exact normalization with integer math is not possible, so this method returns an
    /// approximation that is close enough for most use cases, such as calculating directions or
    /// distances.
    ///
    /// The result is a vector with the same direction as `self`, but with a magnitude as close to
    /// `1` as possible; the result is not guaranteed to have a magnitude of exactly `1`.
    #[must_use]
    pub fn normalized_approx(&self) -> Self {
        if self == &Self::ORIGIN {
            Self::ORIGIN
        } else {
            let gcd = internal::gcd(self.x, self.y);
            Self {
                x: self.x / gcd,
                y: self.y / gcd,
            }
        }
    }

    /// Compares two positions in row-major order (y primary, then x).
    ///
    /// This is the default [`Ord`] ordering for [`Pos`]. Provided as an explicit method for use
    /// when the ordering matters.
    ///
    /// For lexicographic (x-primary) ordering, use [`Pos::cmp_lexicographic`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// // Row-major: y first, then x — matches Ord::cmp
    /// assert_eq!(
    ///     Pos::new(1, 2).cmp_row_major(&Pos::new(0, 3)),
    ///     Pos::new(1, 2).cmp(&Pos::new(0, 3)),
    /// );
    /// ```
    #[must_use]
    pub fn cmp_row_major(&self, other: &Self) -> core::cmp::Ordering {
        self.cmp(other)
    }

    /// Compares two positions in lexicographic order (x primary, then y).
    ///
    /// Contrast with the default [`Ord`] implementation, which is row-major (y primary).
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// // Lexicographic: x first, then y
    /// assert_eq!(
    ///     Pos::new(1, 2).cmp_lexicographic(&Pos::new(0, 3)),
    ///     core::cmp::Ordering::Greater
    /// );
    /// ```
    #[must_use]
    pub fn cmp_lexicographic(&self, other: &Self) -> core::cmp::Ordering {
        self.x.cmp(&other.x).then(self.y.cmp(&other.y))
    }

    /// Attempts to cast this position to a position with a different integer type `U`.
    ///
    /// ## Errors
    ///
    /// Returns [`TryFromPosError::OutOfRange`] if either coordinate cannot be represented by `U`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::<i32>::new(3, 4);
    /// assert_eq!(p.try_cast::<u8>(), Ok(Pos::<u8>::new(3, 4)));
    ///
    /// let out_of_range = Pos::<i32>::new(-3, 4);
    /// assert!(out_of_range.try_cast::<u8>().is_err());
    /// ```
    pub fn try_cast<U: Int + TryFrom<T>>(self) -> Result<Pos<U>, TryFromPosError> {
        let x = U::try_from(self.x).map_err(|_| TryFromPosError::OutOfRange)?;
        let y = U::try_from(self.y).map_err(|_| TryFromPosError::OutOfRange)?;
        Ok(Pos::new(x, y))
    }

    /// Returns a position with both coordinates set to `v`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::splat(5), Pos::new(5, 5));
    /// ```
    #[must_use]
    pub const fn splat(v: T) -> Self {
        Self { x: v, y: v }
    }

    /// Returns the component-wise minimum of `self` and `other`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::new(1, 4).min(Pos::new(3, 2)), Pos::new(1, 2));
    /// ```
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self {
            x: core::cmp::min(self.x, other.x),
            y: core::cmp::min(self.y, other.y),
        }
    }

    /// Returns the component-wise maximum of `self` and `other`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::new(1, 4).max(Pos::new(3, 2)), Pos::new(3, 4));
    /// ```
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self {
            x: core::cmp::max(self.x, other.x),
            y: core::cmp::max(self.y, other.y),
        }
    }

    /// Returns `self` with each component clamped to the `[min, max]` range.
    ///
    /// ## Panics
    ///
    /// Panics if `min.x > max.x` or `min.y > max.y`, matching [`Ord::clamp`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::new(5, -5);
    /// assert_eq!(p.clamp(Pos::new(0, 0), Pos::new(10, 10)), Pos::new(5, 0));
    /// ```
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            x: self.x.clamp(min.x, max.x),
            y: self.y.clamp(min.y, max.y),
        }
    }

    /// Returns the component-wise absolute value of `self`.
    ///
    /// For unsigned integer types, this is always `self`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::new(-3, 4).abs(), Pos::new(3, 4));
    /// ```
    #[must_use]
    pub fn abs(self) -> Self {
        Self {
            x: self.x.abs(),
            y: self.y.abs(),
        }
    }

    /// Returns the [dot product][] of `self` and `other`.
    ///
    /// [dot product]: https://en.wikipedia.org/wiki/Dot_product
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// assert_eq!(Pos::new(1, 2).dot(Pos::new(3, 4)), 11);
    /// ```
    #[must_use]
    pub fn dot(self, other: Self) -> T {
        self.x * other.x + self.y * other.y
    }

    /// Clamps `self` to the closest position inside `bounds`.
    ///
    /// Honors the half-open convention used by [`Rect::contains_pos`] and [`Rect::overlaps`]:
    /// [`Rect::right`] and [`Rect::bottom`] are one past the last valid cell, so the result is
    /// clamped into `[bounds.left(), bounds.right() - 1]` on the x-axis and
    /// `[bounds.top(), bounds.bottom() - 1]` on the y-axis. [`Pos::clamp`] is the wrong tool for
    /// this, since it would clamp `x`/`y` up to (and including) `bounds.bottom_right()`, which is
    /// one cell past the rectangle's actual bounds.
    ///
    /// If `bounds` is empty (zero width and/or zero height), there is no valid cell to clamp
    /// into on that axis, so the corresponding coordinate falls back to [`Rect::top_left`]'s
    /// coordinate on that axis instead.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect};
    ///
    /// let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
    ///
    /// // Already inside `bounds`.
    /// assert_eq!(Pos::new(5, 5).clamp_within(bounds), Pos::new(5, 5));
    ///
    /// // Off each edge, clamps to the closest valid cell.
    /// assert_eq!(Pos::new(-5, 5).clamp_within(bounds), Pos::new(0, 5));
    /// assert_eq!(Pos::new(15, 5).clamp_within(bounds), Pos::new(9, 5));
    ///
    /// // `bottom_right()` is exclusive, so it clamps to the last valid cell, not itself.
    /// assert_eq!(bounds.bottom_right().clamp_within(bounds), Pos::new(9, 9));
    ///
    /// // An empty `bounds` falls back to `top_left()` on the degenerate axis.
    /// let empty = Rect::from_ltrb(2, 3, 2, 8).unwrap();
    /// assert_eq!(Pos::new(100, 5).clamp_within(empty), Pos::new(2, 5));
    /// ```
    #[must_use]
    pub fn clamp_within(self, bounds: Rect<T>) -> Self {
        let x = if bounds.width() == T::ZERO {
            bounds.left()
        } else {
            let hi = core::cmp::max(bounds.right().saturating_sub(T::ONE), bounds.left());
            core::cmp::min(core::cmp::max(self.x, bounds.left()), hi)
        };
        let y = if bounds.height() == T::ZERO {
            bounds.top()
        } else {
            let hi = core::cmp::max(bounds.bottom().saturating_sub(T::ONE), bounds.top());
            core::cmp::min(core::cmp::max(self.y, bounds.top()), hi)
        };
        Self { x, y }
    }

    /// Adds a signed `delta` to `self`, saturating at `T::MIN`/`T::MAX` on each axis instead of
    /// wrapping or underflowing/overflowing.
    ///
    /// This is useful when `T` is an unsigned type (or a narrower signed type than `i32`) and the
    /// delta may be negative or may exceed the range of `T`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::<u16>::new(5, 5);
    /// assert_eq!(
    ///     p.saturating_add_signed(Pos::new(-10, 100_000)),
    ///     Pos::new(0, u16::MAX)
    /// );
    ///
    /// let p = Pos::<i8>::new(0, 0);
    /// assert_eq!(
    ///     p.saturating_add_signed(Pos::new(-200, 200)),
    ///     Pos::new(i8::MIN, i8::MAX)
    /// );
    /// ```
    #[must_use]
    pub fn saturating_add_signed(self, delta: Pos<i32>) -> Self {
        Self {
            x: saturating_apply_magnitude(self.x, to_magnitude(delta.x), delta.x < 0),
            y: saturating_apply_magnitude(self.y, to_magnitude(delta.y), delta.y < 0),
        }
    }

    /// Returns the component-wise saturating addition of `self` and `other`.
    ///
    /// Saturates at `T::MIN`/`T::MAX` instead of wrapping or overflowing.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::<u8>::new(250, 10);
    /// assert_eq!(p.saturating_add(Pos::new(10, 10)), Pos::new(u8::MAX, 20));
    /// ```
    #[must_use]
    pub fn saturating_add(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_add(other.x),
            y: self.y.saturating_add(other.y),
        }
    }

    /// Returns the component-wise saturating subtraction of `self` and `other`.
    ///
    /// Saturates at `T::MIN`/`T::MAX` instead of wrapping or underflowing.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::<u8>::new(5, 10);
    /// assert_eq!(p.saturating_sub(Pos::new(10, 5)), Pos::new(0, 5));
    /// ```
    #[must_use]
    pub fn saturating_sub(self, other: Self) -> Self {
        Self {
            x: self.x.saturating_sub(other.x),
            y: self.y.saturating_sub(other.y),
        }
    }
}

impl<T: SignedInt> Pos<T> {
    /// A unit vector of length `1` in the negative x-direction, i.e. `(-1, 0)`.
    ///
    /// This is the negation of [`Pos::X`].
    ///
    /// Useful in combination with [`ops::Mul`] or [`ops::MulAssign`] to scale the vector.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::NEG_X * 5; // Scales the unit vector by 5
    /// assert_eq!(p, Pos::new(-5, 0));
    ///
    /// let mut q = Pos::NEG_X;
    /// q *= 3; // Scales the unit vector by 3
    /// assert_eq!(q, Pos::new(-3, 0));
    /// ```
    pub const NEG_X: Self = Self {
        x: T::NEG_ONE,
        y: T::ZERO,
    };

    /// A unit vector of length `1` in the negative y-direction, i.e. `(0, -1)`.
    ///
    /// This is the negation of [`Pos::Y`].
    ///
    /// Useful in combination with [`ops::Mul`] or [`ops::MulAssign`] to scale the vector.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Pos;
    ///
    /// let p = Pos::NEG_Y * 5; // Scales the unit vector by 5
    /// assert_eq!(p, Pos::new(0, -5));
    ///
    /// let mut q = Pos::NEG_Y;
    /// q *= 3; // Scales the unit vector by 3
    /// assert_eq!(q, Pos::new(0, -3));
    /// ```
    pub const NEG_Y: Self = Self {
        x: T::ZERO,
        y: T::NEG_ONE,
    };
}

impl<T: Int> Display for Pos<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl<T: Int> PartialOrd for Pos<T> {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Int> Ord for Pos<T> {
    /// Compares two positions in **row-major** order (y primary, then x).
    ///
    /// This is the natural ordering for grid iteration: top-to-bottom, left-to-right within each
    /// row. For lexicographic (x-primary) ordering, use [`Pos::cmp_lexicographic`].
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl<T: Int> Default for Pos<T> {
    fn default() -> Self {
        Self::ORIGIN
    }
}

impl<T: SignedInt> ops::Neg for Pos<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T: Int> ops::Add<Self> for Pos<T> {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Int> ops::AddAssign<Self> for Pos<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Int> ops::Sub<Self> for Pos<T> {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Int> ops::SubAssign<Self> for Pos<T> {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Int> ops::Mul<T> for Pos<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl<T: Int> ops::MulAssign<T> for Pos<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
    }
}

impl<T: Int> ops::Mul<Self> for Pos<T> {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Int> ops::MulAssign<Self> for Pos<T> {
    fn mul_assign(&mut self, rhs: Self) {
        self.x *= rhs.x;
        self.y *= rhs.y;
    }
}

impl<T: Int> ops::Div<T> for Pos<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl<T: Int> ops::DivAssign<T> for Pos<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
    }
}

impl<T: Int> ops::Div<Self> for Pos<T> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Int> ops::DivAssign<Self> for Pos<T> {
    fn div_assign(&mut self, rhs: Self) {
        self.x /= rhs.x;
        self.y /= rhs.y;
    }
}

impl<T: Int> ops::Rem<T> for Pos<T> {
    type Output = Self;

    fn rem(self, rhs: T) -> Self::Output {
        Self {
            x: self.x % rhs,
            y: self.y % rhs,
        }
    }
}

impl<T: Int> ops::RemAssign<T> for Pos<T> {
    fn rem_assign(&mut self, rhs: T) {
        self.x %= rhs;
        self.y %= rhs;
    }
}

impl<T: Int> ops::Rem<Self> for Pos<T> {
    type Output = Self;

    fn rem(self, rhs: Self) -> Self::Output {
        Self {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}

impl<T: Int> ops::RemAssign<Self> for Pos<T> {
    fn rem_assign(&mut self, rhs: Self) {
        self.x %= rhs.x;
        self.y %= rhs.y;
    }
}

macro_rules! impl_scalar_mul {
    ($($t:ty),*) => {
        $(
            impl ops::Mul<Pos<$t>> for $t {
                type Output = Pos<$t>;

                /// Scales `rhs` by `self`, i.e. `n * pos`.
                ///
                /// Equivalent to `rhs * self`; provided for parity with `pos * n`.
                fn mul(self, rhs: Pos<$t>) -> Pos<$t> {
                    rhs * self
                }
            }
        )*
    };
}

#[rustfmt::skip]
impl_scalar_mul!(
    i8, i16, i32, i64, i128, isize,
    u8, u16, u32, u64, u128, usize
);

impl<T: Int> From<(T, T)> for Pos<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T: Int> From<Pos<T>> for (T, T) {
    fn from(pos: Pos<T>) -> Self {
        (pos.x, pos.y)
    }
}

impl<T: Int> From<[T; 2]> for Pos<T> {
    fn from(value: [T; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

impl<T: Int> From<Pos<T>> for [T; 2] {
    fn from(pos: Pos<T>) -> Self {
        [pos.x, pos.y]
    }
}

/// An error type for when a `Pos<T>` cannot be converted to another type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromPosError {
    /// The value is out of range for the target type.
    OutOfRange,
}

impl Display for TryFromPosError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::OutOfRange => write!(f, "value is out of range for the target type"),
        }
    }
}

impl core::error::Error for TryFromPosError {}

/// A position using `u16` coordinates — the natural type for terminal grids.
pub type Pos16 = Pos<u16>;

/// A position using `i32` coordinates — useful for signed offsets and deltas.
pub type PosI = Pos<i32>;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use crate::Size;
    use alloc::string::ToString;

    #[test]
    fn try_from_pos_error_display() {
        assert_eq!(
            TryFromPosError::OutOfRange.to_string(),
            "value is out of range for the target type"
        );
    }

    #[test]
    fn try_from_pos_error_is_error() {
        fn assert_error<E: core::error::Error>(_: &E) {}
        assert_error(&TryFromPosError::OutOfRange);
    }

    #[test]
    fn layout_is_c_struct() {
        // Verifies that Pos and a #[repr(C)] struct with the same fields share the same
        // size and field offsets, ensuring the documented C layout guarantee holds.
        use core::mem::{offset_of, size_of};

        #[repr(C)]
        struct CPos {
            x: i32,
            y: i32,
        }

        assert_eq!(size_of::<Pos<i32>>(), size_of::<CPos>());
        assert_eq!(offset_of!(Pos<i32>, x), offset_of!(CPos, x));
        assert_eq!(offset_of!(Pos<i32>, y), offset_of!(CPos, y));
    }

    #[test]
    fn pos_macro() {
        const P: Pos<i32> = pos!(3, 4);
        assert_eq!(P.x, 3);
        assert_eq!(P.y, 4);
    }

    #[test]
    fn ord_row_major() {
        // Row-major: y primary, then x
        assert!(Pos::new(1, 2) < Pos::new(1, 3)); // y: 2 < 3
        assert!(Pos::new(1, 2) < Pos::new(2, 2)); // y equal, x: 1 < 2
        assert!(Pos::new(0, 3) > Pos::new(1, 2)); // y: 3 > 2
        assert!(Pos::new(2, 1) < Pos::new(1, 2)); // y: 1 < 2
    }

    #[test]
    fn cmp_row_major_y_primary() {
        // Same x, different y: row-major puts smaller y first
        assert_eq!(
            Pos::new(5, 2).cmp_row_major(&Pos::new(5, 3)),
            core::cmp::Ordering::Less
        );
    }

    #[test]
    fn cmp_row_major_x_secondary() {
        // Same y: falls through to x comparison
        assert_eq!(
            Pos::new(1, 3).cmp_row_major(&Pos::new(2, 3)),
            core::cmp::Ordering::Less
        );
    }

    #[test]
    fn cmp_row_major_matches_ord() {
        // cmp_row_major delegates to Ord::cmp
        let a = Pos::new(1, 2);
        let b = Pos::new(0, 3);
        assert_eq!(a.cmp_row_major(&b), a.cmp(&b));
    }

    #[test]
    fn cmp_lexicographic_x_primary() {
        // Same y, different x: lexicographic puts smaller x first
        assert_eq!(
            Pos::new(2, 5).cmp_lexicographic(&Pos::new(1, 5)),
            core::cmp::Ordering::Greater
        );
    }

    #[test]
    fn cmp_lexicographic_y_secondary() {
        // Same x: falls through to y comparison
        assert_eq!(
            Pos::new(3, 1).cmp_lexicographic(&Pos::new(3, 2)),
            core::cmp::Ordering::Less
        );
    }

    #[test]
    fn cmp_lexicographic_differs_from_ord() {
        // Lexicographic (x first) vs row-major (y first)
        let a = Pos::new(1, 2);
        let b = Pos::new(0, 3);
        assert_eq!(a.cmp_lexicographic(&b), core::cmp::Ordering::Greater); // x: 1 > 0
        assert_eq!(a.cmp(&b), core::cmp::Ordering::Less); // y: 2 < 3
    }

    #[test]
    fn cmp_row_major_equal() {
        assert_eq!(
            Pos::new(4, 5).cmp_row_major(&Pos::new(4, 5)),
            core::cmp::Ordering::Equal
        );
    }

    #[test]
    fn generic_t_defaults_to_i32() {
        let p: Pos = Pos::default();
        assert_eq!(p, Pos::<i32>::ORIGIN);
    }

    #[test]
    fn origin_is_0_0() {
        assert_eq!(Pos::ORIGIN, Pos::new(0, 0));
    }

    #[test]
    fn zero_is_origin() {
        assert_eq!(Pos::<i32>::ZERO, Pos::ORIGIN);
    }

    #[test]
    fn splat() {
        assert_eq!(Pos::splat(5), Pos::new(5, 5));
    }

    #[test]
    fn min() {
        assert_eq!(Pos::new(1, 4).min(Pos::new(3, 2)), Pos::new(1, 2));
    }

    #[test]
    fn max() {
        assert_eq!(Pos::new(1, 4).max(Pos::new(3, 2)), Pos::new(3, 4));
    }

    #[test]
    fn clamp() {
        let p = Pos::new(5, -5);
        assert_eq!(p.clamp(Pos::new(0, 0), Pos::new(10, 10)), Pos::new(5, 0));
    }

    #[test]
    fn abs() {
        assert_eq!(Pos::new(-3, 4).abs(), Pos::new(3, 4));
    }

    #[test]
    fn abs_unsigned_is_self() {
        let p: Pos<u32> = Pos::new(3, 4);
        assert_eq!(p.abs(), p);
    }

    #[test]
    fn dot() {
        assert_eq!(Pos::new(1, 2).dot(Pos::new(3, 4)), 11);
    }

    #[test]
    fn rem_scalar() {
        let p = Pos::new(7, 9) % 3;
        assert_eq!(p, Pos::new(1, 0));
    }

    #[test]
    fn rem_assign_scalar() {
        let mut p = Pos::new(7, 9);
        p %= 3;
        assert_eq!(p, Pos::new(1, 0));
    }

    #[test]
    fn rem_pos() {
        let p1 = Pos::new(7, 9);
        let p2 = Pos::new(3, 4);
        assert_eq!(p1 % p2, Pos::new(1, 1));
    }

    #[test]
    fn rem_assign_pos() {
        let mut p1 = Pos::new(7, 9);
        let p2 = Pos::new(3, 4);
        p1 %= p2;
        assert_eq!(p1, Pos::new(1, 1));
    }

    #[test]
    fn min_is_min_min() {
        assert_eq!(Pos::MIN, Pos::new(i32::MIN, i32::MIN));
    }

    #[test]
    fn max_is_max_max() {
        assert_eq!(Pos::MAX, Pos::new(i32::MAX, i32::MAX));
    }

    #[test]
    fn x_is_1_0() {
        assert_eq!(Pos::X, Pos::new(1, 0));
    }

    #[test]
    fn y_is_0_1() {
        assert_eq!(Pos::Y, Pos::new(0, 1));
    }

    #[test]
    fn new_x_y() {
        let p = Pos::new(3, 4);
        assert_eq!(p.x, 3);
        assert_eq!(p.y, 4);
    }

    #[test]
    fn default_is_origin() {
        let p: Pos<i32> = Pos::default();
        assert_eq!(p, Pos::ORIGIN);
    }

    #[test]
    fn negate() {
        let p = Pos::new(3, 4);
        assert_eq!(-p, Pos::new(-3, -4));
    }

    #[test]
    fn mul_by_scalar() {
        let p = Pos::new(3, 4) * 2;
        assert_eq!(p, Pos::new(6, 8));
    }

    #[test]
    fn scalar_mul_by_pos() {
        let p = 2 * Pos::new(3, 4);
        assert_eq!(p, Pos::new(6, 8));
    }

    #[test]
    fn scalar_mul_matches_pos_mul() {
        let p = Pos::new(3, 4);
        assert_eq!(2 * p, p * 2);
    }

    #[test]
    fn scalar_mul_unsigned() {
        let p: Pos<u32> = Pos::new(3, 4);
        assert_eq!(2u32 * p, Pos::new(6, 8));
    }

    #[test]
    fn mul_assign_by_scalar() {
        let mut p = Pos::new(3, 4);
        p *= 2;
        assert_eq!(p, Pos::new(6, 8));
    }

    #[test]
    fn from_tuple() {
        let pos = Pos::from((3, 4));
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 4);
    }

    #[test]
    fn from_array() {
        let pos = Pos::from([3, 4]);
        assert_eq!(pos.x, 3);
        assert_eq!(pos.y, 4);
    }

    #[test]
    fn into_tuple() {
        let pos = Pos::new(3, 4);
        let tuple: (i32, i32) = pos.into();
        assert_eq!(tuple, (3, 4));
    }

    #[test]
    fn into_array() {
        let pos = Pos::new(3, 4);
        let array: [i32; 2] = pos.into();
        assert_eq!(array, [3, 4]);
    }

    #[test]
    fn try_cast_ok() {
        let source: Pos<u8> = Pos::new(3, 4);
        let convert = source.try_cast::<i32>().unwrap();
        assert_eq!(convert.x, 3);
        assert_eq!(convert.y, 4);
    }

    #[test]
    fn try_cast_out_of_range() {
        let source: Pos<u16> = Pos::new(7000, 8000);
        let result = source.try_cast::<u8>();
        assert!(result.is_err());
    }

    #[test]
    fn add_pos() {
        let p1 = Pos::new(3, 4);
        let p2 = Pos::new(1, 2);
        assert_eq!(p1 + p2, Pos::new(4, 6));
    }

    #[test]
    fn add_assign_pos() {
        let mut p1 = Pos::new(3, 4);
        let p2 = Pos::new(1, 2);
        p1 += p2;
        assert_eq!(p1, Pos::new(4, 6));
    }

    #[test]
    fn sub_pos() {
        let p1 = Pos::new(3, 4);
        let p2 = Pos::new(1, 2);
        assert_eq!(p1 - p2, Pos::new(2, 2));
    }

    #[test]
    fn sub_assign_pos() {
        let mut p1 = Pos::new(3, 4);
        let p2 = Pos::new(1, 2);
        p1 -= p2;
        assert_eq!(p1, Pos::new(2, 2));
    }

    #[test]
    fn into_size() {
        let pos = Pos::new(3, 4);
        let size = Size::try_from(pos).unwrap();
        assert_eq!(size.width, 3);
        assert_eq!(size.height, 4);
    }

    #[test]
    fn into_size_wrapped() {
        let pos = Pos::new(-3, -4);
        let size = Size::try_from(pos);
        assert!(size.is_err());
    }

    #[test]
    fn normalized() {
        assert_eq!(Pos::new(0, 0).normalized_approx(), Pos::ORIGIN);
        assert_eq!(Pos::new(1, 0).normalized_approx(), Pos::X);
        assert_eq!(Pos::new(0, 1).normalized_approx(), Pos::Y);
        assert_eq!(Pos::new(2, 0).normalized_approx(), Pos::X);
        assert_eq!(Pos::new(0, 2).normalized_approx(), Pos::Y);
        assert_eq!(
            Pos::new(6, 8).normalized_approx(),
            Pos::new(3, 4).normalized_approx()
        );
    }

    #[test]
    fn mul_scalar() {
        let p = Pos::new(3, 4) * 2;
        assert_eq!(p, Pos::new(6, 8));
    }

    #[test]
    fn mul_assign_scalar() {
        let mut p = Pos::new(3, 4);
        p *= 2;
        assert_eq!(p, Pos::new(6, 8));
    }

    #[test]
    fn mul_pos() {
        let p1 = Pos::new(3, 4);
        let p2 = Pos::new(2, 3);
        assert_eq!(p1 * p2, Pos::new(6, 12));
    }

    #[test]
    fn mul_assign_pos() {
        let mut p1 = Pos::new(3, 4);
        let p2 = Pos::new(2, 3);
        p1 *= p2;
        assert_eq!(p1, Pos::new(6, 12));
    }

    #[test]
    fn div_scalar() {
        let p = Pos::new(6, 8) / 2;
        assert_eq!(p, Pos::new(3, 4));
    }

    #[test]
    fn div_assign_scalar() {
        let mut p = Pos::new(6, 8);
        p /= 2;
        assert_eq!(p, Pos::new(3, 4));
    }

    #[test]
    fn div_pos() {
        let p1 = Pos::new(6, 8);
        let p2 = Pos::new(2, 4);
        assert_eq!(p1 / p2, Pos::new(3, 2));
    }

    #[test]
    fn div_assign_pos() {
        let mut p1 = Pos::new(6, 8);
        let p2 = Pos::new(2, 4);
        p1 /= p2;
        assert_eq!(p1, Pos::new(3, 2));
    }

    #[test]
    fn pos16_alias() {
        let p: Pos16 = Pos16::new(1, 2);
        assert_eq!(p.x, 1u16);
        assert_eq!(p.y, 2u16);
    }

    #[test]
    fn posi_alias() {
        let p: PosI = PosI::new(1, 2);
        assert_eq!(p.x, 1i32);
        assert_eq!(p.y, 2i32);
    }

    #[test]
    fn clamp_within_already_inside() {
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(Pos::new(5, 5).clamp_within(bounds), Pos::new(5, 5));
    }

    #[test]
    fn clamp_within_off_left_edge() {
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(Pos::new(-5, 5).clamp_within(bounds), Pos::new(0, 5));
    }

    #[test]
    fn clamp_within_off_right_edge() {
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(Pos::new(15, 5).clamp_within(bounds), Pos::new(9, 5));
    }

    #[test]
    fn clamp_within_off_top_edge() {
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(Pos::new(5, -5).clamp_within(bounds), Pos::new(5, 0));
    }

    #[test]
    fn clamp_within_off_bottom_edge() {
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(Pos::new(5, 15).clamp_within(bounds), Pos::new(5, 9));
    }

    #[test]
    fn clamp_within_exclusive_bottom_right_trap() {
        // bounds.bottom_right() is one past the last valid cell; clamping it must land on the
        // last valid cell, not on bottom_right() itself.
        let bounds = Rect::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(bounds.bottom_right().clamp_within(bounds), Pos::new(9, 9));
    }

    #[test]
    fn clamp_within_empty_width_and_height() {
        let bounds = Rect::from_ltrb(2, 3, 2, 3).unwrap();
        assert!(bounds.is_empty());
        assert_eq!(Pos::new(100, 100).clamp_within(bounds), Pos::new(2, 3));
    }

    #[test]
    fn clamp_within_empty_width_only() {
        let bounds = Rect::from_ltrb(2, 3, 2, 8).unwrap();
        assert_eq!(Pos::new(100, 5).clamp_within(bounds), Pos::new(2, 5));
    }

    #[test]
    fn clamp_within_empty_height_only() {
        let bounds = Rect::from_ltrb(2, 3, 8, 3).unwrap();
        assert_eq!(Pos::new(5, 100).clamp_within(bounds), Pos::new(5, 3));
    }

    #[test]
    fn clamp_within_u16_coordinates() {
        let bounds = Rect::<u16>::from_ltrb(0, 0, 10, 10).unwrap();
        assert_eq!(
            Pos::<u16>::new(65535, 65535).clamp_within(bounds),
            Pos::new(9, 9)
        );
    }

    #[test]
    fn saturating_add_pos() {
        let p = Pos::<u8>::new(250, 10);
        assert_eq!(p.saturating_add(Pos::new(10, 10)), Pos::new(u8::MAX, 20));
    }

    #[test]
    fn saturating_sub_pos() {
        let p = Pos::<u8>::new(5, 10);
        assert_eq!(p.saturating_sub(Pos::new(10, 5)), Pos::new(0, 5));
    }

    #[test]
    fn saturating_add_signed_unsigned_saturates_at_zero() {
        let p = Pos::<u16>::new(5, 5);
        assert_eq!(
            p.saturating_add_signed(Pos::new(-10, 100_000)),
            Pos::new(0, u16::MAX)
        );
    }

    #[test]
    fn saturating_add_signed_signed_saturates_both_ends() {
        let p = Pos::<i8>::new(0, 0);
        assert_eq!(
            p.saturating_add_signed(Pos::new(-200, 200)),
            Pos::new(i8::MIN, i8::MAX)
        );
    }

    #[test]
    fn saturating_add_signed_large_magnitude_i32_delta() {
        // i32::MIN's magnitude does not fit in a u32/usize directly via `abs()`, exercise
        // `unsigned_abs()` handling of the largest possible negative delta.
        let p = Pos::<i8>::new(0, 0);
        assert_eq!(
            p.saturating_add_signed(Pos::new(i32::MIN, i32::MAX)),
            Pos::new(i8::MIN, i8::MAX)
        );
    }

    #[test]
    fn saturating_add_signed_exact_no_saturation() {
        // Regression test: a single clamped step would previously under- or over-apply the
        // delta for narrow signed types even when the exact result is representable.
        let p = Pos::<i8>::new(-128, 127);
        assert_eq!(
            p.saturating_add_signed(Pos::new(200, -200)),
            Pos::new(72, -73)
        );
    }

    #[test]
    fn saturating_add_signed_zero_delta_is_identity() {
        let p = Pos::<i32>::new(3, 4);
        assert_eq!(p.saturating_add_signed(Pos::new(0, 0)), p);
    }
}
