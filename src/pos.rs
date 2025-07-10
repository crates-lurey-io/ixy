use core::ops;

use crate::{
    IntoSize,
    int::{Int, SignedInt},
};

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
/// # Layout
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
/// # Ordering
///
/// Points are ordered _lexographically_, or the point with the smaller `x` coordinate comes first.
///
/// If two points have the same `x` coordinate, the point with the smaller `y` coordinate is first.
///
/// ```rust
/// use ixy::Pos;
///
/// assert!(Pos::new(1, 2) < Pos::new(2, 1));
/// assert!(Pos::new(2, 2) > Pos::new(1, 2));
/// ```
///
/// # Examples
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
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
#[repr(C)]
#[allow(private_bounds)]
pub struct Pos<T: Int = i32> {
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
    /// # Examples
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
}

impl<T: SignedInt> Pos<T> {
    /// A unit vector of length `1` in the negative x-direction, i.e. `(-1, 0)`.
    ///
    /// This is the negation of [`Pos::X`].
    ///
    /// Useful in combination with [`ops::Mul`] or [`ops::MulAssign`] to scale the vector.
    ///
    /// # Examples
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
    /// # Examples
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

/// A trait for converting a `Pos<T>` to another type.
pub trait TryFromPos<T: Int>: Sized {
    /// Returns the type that the `Pos<T>` can be converted to.
    ///
    /// # Errors
    ///
    /// If the conversion fails, returns a `TryFromPosError`.
    fn try_from_pos(value: Pos<T>) -> Result<Self, TryFromPosError>;
}

/// A trait for converting a `Pos<T>` to another type.
pub trait TryIntoPos<T: Int>: Sized {
    /// Returns the type that the `Pos<T>` can be converted to.
    ///
    /// # Errors
    ///
    /// If the conversion fails, returns a `TryFromPosError`.
    fn try_into_pos(self) -> Result<Pos<T>, TryFromPosError>;
}

impl<T, U> TryIntoPos<U> for Pos<T>
where
    Pos<U>: TryFromPos<T>,
    U: Int,
    T: Int,
{
    fn try_into_pos(self) -> Result<Pos<U>, TryFromPosError> {
        Pos::<U>::try_from_pos(self)
    }
}

impl<T: Int> AsRef<[T; 2]> for Pos<T> {
    fn as_ref(&self) -> &[T; 2] {
        // SAFETY: Pos<T> is #[repr(C)] and has the same layout as [T; 2]
        unsafe { &*core::ptr::from_ref::<Pos<T>>(self).cast::<[T; 2]>() }
    }
}

impl<T: Int> AsRef<(T, T)> for Pos<T> {
    fn as_ref(&self) -> &(T, T) {
        // SAFETY: Pos<T> is #[repr(C)] and has the same layout as (T, T)
        unsafe { &*core::ptr::from_ref::<Pos<T>>(self).cast::<(T, T)>() }
    }
}

impl<T: Int> AsMut<[T; 2]> for Pos<T> {
    fn as_mut(&mut self) -> &mut [T; 2] {
        // SAFETY: Pos<T> is #[repr(C)] and has the same layout as [T; 2]
        unsafe { &mut *core::ptr::from_mut::<Pos<T>>(self).cast::<[T; 2]>() }
    }
}

impl<T: Int> AsMut<(T, T)> for Pos<T> {
    fn as_mut(&mut self) -> &mut (T, T) {
        // SAFETY: Pos<T> is #[repr(C)] and has the same layout as (T, T)
        unsafe { &mut *(core::ptr::from_mut::<Pos<T>>(self)).cast::<(T, T)>() }
    }
}

/// An error type for when a `Pos<T>` cannot be converted to another type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TryFromPosError {
    /// The value is out of range for the target type.
    OutOfRange,
}

impl<S: Int, T: Int + TryFrom<S>> TryFromPos<S> for Pos<T> {
    fn try_from_pos(value: Pos<S>) -> Result<Self, TryFromPosError> {
        let x = T::try_from(value.x).map_err(|_| TryFromPosError::OutOfRange)?;
        let y = T::try_from(value.y).map_err(|_| TryFromPosError::OutOfRange)?;
        Ok(Pos::new(x, y))
    }
}

impl<T: Int> IntoSize for Pos<T> {
    type Dim = T;

    fn into_size(self) -> crate::Size<Self::Dim> {
        crate::Size {
            width: self.x,
            height: self.y,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn layout_is_c_struct() {
        struct CPos {
            x: i32,
            y: i32,
        }

        let pos = Pos::<i32> { x: 1, y: 2 };

        #[allow(unsafe_code, reason = "Test")]
        let c_pos: CPos = unsafe { core::mem::transmute(pos) };
        assert_eq!(c_pos.x, 1);
        assert_eq!(c_pos.y, 2);
    }

    #[test]
    fn pos_macro() {
        const P: Pos<i32> = pos!(3, 4);
        assert_eq!(P.x, 3);
        assert_eq!(P.y, 4);
    }

    #[test]
    fn ord() {
        assert!(Pos::new(1, 2) < Pos::new(2, 1));
        assert!(Pos::new(1, 2) < Pos::new(1, 3));
        assert!(Pos::new(1, 2) < Pos::new(2, 2));
        assert!(Pos::new(2, 2) > Pos::new(1, 2));
        assert!(Pos::new(2, 1) > Pos::new(1, 2));
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
    fn try_from_pos_ok() {
        let source: Pos<u8> = Pos::new(3, 4);
        let convert = Pos::<i32>::try_from_pos(source).unwrap();
        assert_eq!(convert.x, 3);
        assert_eq!(convert.y, 4);
    }

    #[test]
    fn try_from_pos_out_of_range() {
        let source: Pos<u16> = Pos::new(7000, 8000);
        let result = Pos::<u8>::try_from_pos(source);
        assert!(result.is_err());
    }

    #[test]
    fn try_into_pos_ok() {
        let source: Pos<u8> = Pos::new(3, 4);
        let convert: Pos<i32> = source.try_into_pos().unwrap();
        assert_eq!(convert.x, 3);
        assert_eq!(convert.y, 4);
    }

    #[test]
    fn try_into_pos_out_of_range() {
        let source: Pos<u16> = Pos::new(7000, 8000);
        let result: Result<Pos<u8>, TryFromPosError> = source.try_into_pos();
        assert!(result.is_err());
    }

    #[test]
    fn as_ref_array() {
        let pos = Pos::new(3, 4);
        let arr: &[i32; 2] = pos.as_ref();
        assert_eq!(arr, &[3, 4]);
    }

    #[test]
    fn as_mut_array() {
        let mut pos = Pos::new(3, 4);
        let arr: &mut [i32; 2] = pos.as_mut();
        arr[0] = 5;
        arr[1] = 6;
        assert_eq!(pos, Pos::new(5, 6));
    }

    #[test]
    fn as_ref_tuple() {
        let pos = Pos::new(3, 4);
        let (x, y) = pos.as_ref();
        assert_eq!(x, &3);
        assert_eq!(y, &4);
    }

    #[test]
    fn as_mut_tuple() {
        let mut pos = Pos::new(3, 4);
        let (x, y) = pos.as_mut();
        *x = 5;
        *y = 6;
        assert_eq!(pos, Pos::new(5, 6));
    }
}
