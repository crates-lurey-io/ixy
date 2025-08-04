use core::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

use crate::Rect;

/// Represents a size in 2D space, with `width` and `height`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size {
    /// Width.
    pub width: usize,

    /// Height.
    pub height: usize,
}

impl Size {
    /// Creates a new size.
    #[must_use]
    pub const fn new(width: usize, height: usize) -> Self {
        Self { width, height }
    }

    /// Returns the area of the size (width * height).
    #[must_use]
    pub const fn area(&self) -> usize {
        self.width * self.height
    }
}

impl Add for Size {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }
}

impl AddAssign for Size {
    fn add_assign(&mut self, other: Self) {
        self.width += other.width;
        self.height += other.height;
    }
}

impl Sub for Size {
    type Output = Self;

    fn sub(self, other: Self) -> Self::Output {
        Self {
            width: self.width - other.width,
            height: self.height - other.height,
        }
    }
}

impl SubAssign for Size {
    fn sub_assign(&mut self, other: Self) {
        self.width -= other.width;
        self.height -= other.height;
    }
}

impl Mul for Size {
    type Output = Self;

    fn mul(self, other: Self) -> Self::Output {
        Self {
            width: self.width * other.width,
            height: self.height * other.height,
        }
    }
}

impl Mul<usize> for Size {
    type Output = Self;

    fn mul(self, scalar: usize) -> Self::Output {
        Self {
            width: self.width * scalar,
            height: self.height * scalar,
        }
    }
}

impl MulAssign for Size {
    fn mul_assign(&mut self, other: Self) {
        self.width *= other.width;
        self.height *= other.height;
    }
}

impl MulAssign<usize> for Size {
    fn mul_assign(&mut self, scalar: usize) {
        self.width *= scalar;
        self.height *= scalar;
    }
}

impl Div for Size {
    type Output = Self;

    fn div(self, other: Self) -> Self::Output {
        Self {
            width: self.width / other.width,
            height: self.height / other.height,
        }
    }
}

impl Div<usize> for Size {
    type Output = Self;

    fn div(self, scalar: usize) -> Self::Output {
        Self {
            width: self.width / scalar,
            height: self.height / scalar,
        }
    }
}

impl DivAssign for Size {
    fn div_assign(&mut self, other: Self) {
        self.width /= other.width;
        self.height /= other.height;
    }
}

impl DivAssign<usize> for Size {
    fn div_assign(&mut self, scalar: usize) {
        self.width /= scalar;
        self.height /= scalar;
    }
}

/// A type that has a [`Size`].
pub trait HasSize {
    /// Returns the size of the object.
    fn size(&self) -> Size;

    /// Returns the width of the object.
    fn width(&self) -> usize {
        self.size().width
    }

    /// Returns the height of the object.
    fn height(&self) -> usize {
        self.size().height
    }

    /// Returns a rectangle at `Pos::ORIGIN` where the size is the object's size.
    fn to_rect(&self) -> Rect<usize> {
        Rect::from_ltwh(0, 0, self.width(), self.height())
    }
}

impl HasSize for Size {
    fn size(&self) -> Size {
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
}
