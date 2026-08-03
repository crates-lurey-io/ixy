use core::{
    fmt,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use crate::{Pos, Rect, TryFromPosError, int::Int};

/// Represents a size in 2D space, with `width` and `height`.
///
/// The type parameter `T` is guaranteed to be a built-in Rust integer type, and defaults to
/// `usize`, the natural type for indexing and allocation.
///
/// ## Ordering
///
/// Sizes are ordered lexicographically: `width` first, then `height`.
///
/// ```rust
/// use ixy::Size;
///
/// assert!(Size::new(1, 5) < Size::new(2, 0));   // width-primary: 1 < 2
/// assert!(Size::new(2, 0) < Size::new(2, 1));   // width equal, height: 0 < 1
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Size<T = usize> {
    /// Width.
    pub width: T,

    /// Height.
    pub height: T,
}

impl<T: Int> Size<T> {
    /// Creates a new size.
    #[must_use]
    pub const fn new(width: T, height: T) -> Self {
        Self { width, height }
    }

    /// Returns the area of the size (width * height).
    #[must_use]
    pub fn area(&self) -> T {
        self.width * self.height
    }

    /// Converts the size to a position.
    #[must_use]
    pub const fn to_pos(&self) -> Pos<T> {
        Pos::new(self.width, self.height)
    }

    /// Returns a rectangle at [`Pos::ORIGIN`] with this size.
    ///
    /// This is the `const` inherent equivalent of [`HasSize::to_rect`], for callers that need an
    /// origin rectangle from a `const fn`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Size};
    ///
    /// const AREA: Rect<usize> = Size::new(10, 20).to_rect();
    /// assert_eq!(AREA, Rect::from_ltwh(0, 0, 10, 20));
    /// ```
    #[must_use]
    pub const fn to_rect(&self) -> Rect<T> {
        Rect::from_ltwh(T::ZERO, T::ZERO, self.width, self.height)
    }

    /// Returns the component-wise minimum of `self` and `other`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Size;
    ///
    /// assert_eq!(Size::new(1, 4).min(Size::new(3, 2)), Size::new(1, 2));
    /// ```
    #[must_use]
    pub fn min(self, other: Self) -> Self {
        Self {
            width: core::cmp::min(self.width, other.width),
            height: core::cmp::min(self.height, other.height),
        }
    }

    /// Returns the component-wise maximum of `self` and `other`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Size;
    ///
    /// assert_eq!(Size::new(1, 4).max(Size::new(3, 2)), Size::new(3, 4));
    /// ```
    #[must_use]
    pub fn max(self, other: Self) -> Self {
        Self {
            width: core::cmp::max(self.width, other.width),
            height: core::cmp::max(self.height, other.height),
        }
    }

    /// Returns `self` with each component clamped to the `[min, max]` range.
    ///
    /// ## Panics
    ///
    /// Panics if `min.width > max.width` or `min.height > max.height`, matching [`Ord::clamp`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Size;
    ///
    /// let size = Size::new(80, 1);
    /// assert_eq!(size.clamp(Size::new(10, 10), Size::new(40, 40)), Size::new(40, 10));
    /// ```
    #[must_use]
    pub fn clamp(self, min: Self, max: Self) -> Self {
        Self {
            width: self.width.clamp(min.width, max.width),
            height: self.height.clamp(min.height, max.height),
        }
    }
}

impl<T: Int> Default for Size<T> {
    /// Returns a size of `(0, 0)`.
    fn default() -> Self {
        Self {
            width: T::ZERO,
            height: T::ZERO,
        }
    }
}

impl<T: Int> Add for Size<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl<T: Int> AddAssign for Size<T> {
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl<T: Int> Sub for Size<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl<T: Int> SubAssign for Size<T> {
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl<T: Int> Mul for Size<T> {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl<T: Int> Mul<T> for Size<T> {
    type Output = Self;

    fn mul(self, scalar: T) -> Self::Output {
        Self {
            width: self.width * scalar,
            height: self.height * scalar,
        }
    }
}

impl<T: Int> MulAssign for Size<T> {
    fn mul_assign(&mut self, other: Self) {
        self.width *= other.width;
        self.height *= other.height;
    }
}

impl<T: Int> MulAssign<T> for Size<T> {
    fn mul_assign(&mut self, scalar: T) {
        self.width *= scalar;
        self.height *= scalar;
    }
}

impl<T: Int> Div for Size<T> {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl<T: Int> Div<T> for Size<T> {
    type Output = Self;

    fn div(self, scalar: T) -> Self::Output {
        Self {
            width: self.width / scalar,
            height: self.height / scalar,
        }
    }
}

impl<T: Int> DivAssign for Size<T> {
    fn div_assign(&mut self, other: Self) {
        self.width /= other.width;
        self.height /= other.height;
    }
}

impl<T: Int> DivAssign<T> for Size<T> {
    fn div_assign(&mut self, scalar: T) {
        self.width /= scalar;
        self.height /= scalar;
    }
}

impl<T: Int> fmt::Display for Size<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}×{}", self.width, self.height)
    }
}

impl<T: Int> From<Size<T>> for Pos<T> {
    fn from(size: Size<T>) -> Self {
        size.to_pos()
    }
}

impl<T: Int> From<(T, T)> for Size<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}

impl<T: Int> From<Size<T>> for (T, T) {
    fn from(size: Size<T>) -> Self {
        (size.width, size.height)
    }
}

impl<T: Int> From<[T; 2]> for Size<T> {
    fn from(value: [T; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}

impl<T: Int> From<Size<T>> for [T; 2] {
    fn from(size: Size<T>) -> Self {
        [size.width, size.height]
    }
}

impl<T: Int> TryFrom<Pos<T>> for Size<T> {
    type Error = TryFromPosError;

    /// Converts a position to a size, failing if either coordinate is negative.
    ///
    /// ## Errors
    ///
    /// Returns [`TryFromPosError::OutOfRange`] if `pos.x` or `pos.y` is negative.
    fn try_from(pos: Pos<T>) -> Result<Self, Self::Error> {
        if pos.x < T::ZERO || pos.y < T::ZERO {
            return Err(TryFromPosError::OutOfRange);
        }
        Ok(Self::new(pos.x, pos.y))
    }
}

/// A type that has a [`Size<T>`].
///
/// The type parameter `T` defaults to `usize`, matching [`Size`]'s default.
#[allow(private_bounds)]
pub trait HasSize<T: Int = usize> {
    /// Returns the size of the object.
    #[must_use]
    fn size(&self) -> Size<T>;

    /// Returns the width of the object.
    #[must_use]
    fn width(&self) -> T {
        self.size().width
    }

    /// Returns the height of the object.
    #[must_use]
    fn height(&self) -> T {
        self.size().height
    }

    /// Returns a rectangle at `Pos::ORIGIN` where the size is the object's size.
    ///
    /// [`Size`] additionally provides an inherent `const fn` of the same name, and [`Rect`] the
    /// equivalent [`Rect::at_origin`].
    #[must_use]
    fn to_rect(&self) -> Rect<T> {
        Rect::from_ltwh(T::ZERO, T::ZERO, self.width(), self.height())
    }
}

impl<T: Int> HasSize<T> for Size<T> {
    fn size(&self) -> Self {
        *self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn size() {
        let size = Size {
            width: 10,
            height: 20,
        };
        assert_eq!(size.width(), 10);
        assert_eq!(size.height(), 20);
    }

    #[test]
    fn to_rect() {
        let size = Size::new(10, 20);
        let rect = size.to_rect();
        assert_eq!(rect.left(), 0);
        assert_eq!(rect.top(), 0);
        assert_eq!(rect.right(), 10);
        assert_eq!(rect.bottom(), 20);
    }

    #[test]
    fn to_rect_is_const() {
        const RECT: Rect<u16> = Size::new(10u16, 20u16).to_rect();
        assert_eq!(RECT, Rect::from_ltwh(0, 0, 10, 20));
    }

    #[test]
    fn to_rect_inherent_matches_has_size() {
        let size = Size::new(10, 20);
        assert_eq!(size.to_rect(), HasSize::to_rect(&size));
    }

    #[test]
    fn min() {
        assert_eq!(Size::new(1, 4).min(Size::new(3, 2)), Size::new(1, 2));
    }

    #[test]
    fn max() {
        assert_eq!(Size::new(1, 4).max(Size::new(3, 2)), Size::new(3, 4));
    }

    #[test]
    fn clamp() {
        let size = Size::new(80, 1);
        assert_eq!(
            size.clamp(Size::new(10, 10), Size::new(40, 40)),
            Size::new(40, 10)
        );
    }

    #[test]
    fn clamp_within_range_is_unchanged() {
        let size = Size::new(20, 20);
        assert_eq!(size.clamp(Size::new(10, 10), Size::new(40, 40)), size);
    }

    #[test]
    fn add_size_size() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(5, 15);
        let result = size1 + size2;
        assert_eq!(result, Size::new(15, 35));
    }

    #[test]
    fn add_assign_size() {
        let mut size = Size::new(10, 20);
        size += Size::new(5, 15);
        assert_eq!(size, Size::new(15, 35));
    }

    #[test]
    fn sub_size_size() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(5, 15);
        let result = size1 - size2;
        assert_eq!(result, Size::new(5, 5));
    }

    #[test]
    fn sub_assign_size() {
        let mut size = Size::new(10, 20);
        size -= Size::new(5, 15);
        assert_eq!(size, Size::new(5, 5));
    }

    #[test]
    fn mul_size_size() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(2, 3);
        let result = size1 * size2;
        assert_eq!(result, Size::new(20, 60));
    }

    #[test]
    fn mul_assign_size() {
        let mut size = Size::new(10, 20);
        size *= Size::new(2, 3);
        assert_eq!(size, Size::new(20, 60));
    }

    #[test]
    fn mul_size_scalar() {
        let size = Size::new(10, 20);
        let scalar = 2;
        let result = size * scalar;
        assert_eq!(result, Size::new(20, 40));
    }

    #[test]
    fn mul_assign_size_scalar() {
        let mut size = Size::new(10, 20);
        size *= 2;
        assert_eq!(size, Size::new(20, 40));
    }

    #[test]
    fn div_size_size() {
        let size1 = Size::new(10, 20);
        let size2 = Size::new(2, 4);
        let result = size1 / size2;
        assert_eq!(result, Size::new(5, 5));
    }

    #[test]
    fn div_assign_size() {
        let mut size = Size::new(10, 20);
        size /= Size::new(2, 4);
        assert_eq!(size, Size::new(5, 5));
    }

    #[test]
    fn div_size_scalar() {
        let size = Size::new(10, 20);
        let scalar = 2;
        let result = size / scalar;
        assert_eq!(result, Size::new(5, 10));
    }

    #[test]
    fn div_assign_size_scalar() {
        let mut size = Size::new(10, 20);
        size /= 2;
        assert_eq!(size, Size::new(5, 10));
    }

    #[test]
    fn size_area() {
        let size = Size::new(10, 20);
        assert_eq!(size.area(), 200);
    }

    #[test]
    fn from_size_for_pos() {
        let size = Size::new(10, 20);
        let pos: Pos<usize> = size.into();
        assert_eq!(pos, Pos::new(10, 20));
    }

    #[test]
    fn try_from_pos_for_size_ok() {
        let pos = Pos::new(3, 4);
        let size = Size::try_from(pos).unwrap();
        assert_eq!(size, Size::new(3, 4));
    }

    #[test]
    fn try_from_pos_for_size_negative() {
        let pos = Pos::new(-3, -4);
        assert!(Size::try_from(pos).is_err());
    }

    #[test]
    fn generic_size_u16() {
        let size: Size<u16> = Size::new(10, 20);
        assert_eq!(size.width, 10u16);
        assert_eq!(size.area(), 200u16);
    }

    #[test]
    fn default_is_zero_zero() {
        let size: Size<i32> = Size::default();
        assert_eq!(size, Size::new(0, 0));
    }

    #[test]
    fn ord_width_primary() {
        // Width-primary: width first, then height.
        assert!(Size::new(1, 5) < Size::new(2, 0));
        assert!(Size::new(2, 0) < Size::new(2, 1));
        assert!(Size::new(3, 0) > Size::new(2, 5));
    }

    #[test]
    fn partial_ord_matches_ord() {
        let a = Size::new(1, 2);
        let b = Size::new(1, 3);
        assert_eq!(a.partial_cmp(&b), Some(a.cmp(&b)));
    }

    #[test]
    fn from_tuple() {
        let size = Size::from((3, 4));
        assert_eq!(size, Size::new(3, 4));
    }

    #[test]
    fn into_tuple() {
        let size = Size::new(3, 4);
        let tuple: (i32, i32) = size.into();
        assert_eq!(tuple, (3, 4));
    }

    #[test]
    fn from_array() {
        let size = Size::from([3, 4]);
        assert_eq!(size, Size::new(3, 4));
    }

    #[test]
    fn into_array() {
        let size = Size::new(3, 4);
        let array: [i32; 2] = size.into();
        assert_eq!(array, [3, 4]);
    }

    #[test]
    fn into_size_from_tuple_literal() {
        fn takes_size(size: impl Into<Size<u16>>) -> Size<u16> {
            size.into()
        }
        assert_eq!(takes_size((80u16, 25u16)), Size::new(80, 25));
    }
}
