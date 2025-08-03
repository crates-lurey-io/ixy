use core::ops;

use crate::{
    HasSize, Pos, Size,
    index::{ColMajor, Layout, RowMajor},
    int::Int,
};

/// A macro that creates a rectangle with the given coordinates.
///
/// Unlike [`Rect::from_tlbr`] or [`Rect::from_ltrb`], this macro is infallible as it guarantees
/// that the coordinates form a valid rectangle, by re-arranging them if necessary; i.e. swapping
/// either the left and right coordinates, or the top and bottom coordinates.
///
/// ## Examples
///
/// ```rust
/// use ixy::{rect, Pos};
///
/// let rect_ltrb = rect!(1, 2, 3, 4);
/// let rect_tlbr = rect!(Pos::new(1, 2), Pos::new(3, 4));
/// ```
#[macro_export]
macro_rules! rect {
    ($tl: expr, $br: expr) => {{
        let tl = $tl;
        let br = $br;
        let l = if tl.x < br.x { tl.x } else { br.x };
        let t = if tl.y < br.y { tl.y } else { br.y };
        let r = if tl.x < br.x { br.x } else { tl.x };
        let b = if tl.y < br.y { br.y } else { tl.y };
        unsafe { $crate::Rect::from_ltrb_unchecked(l, t, r, b) }
    }};
    ($l:expr, $t:expr, $r:expr, $b:expr) => {{
        let l = if $l < $r { $l } else { $r };
        let t = if $t < $b { $t } else { $b };
        let r = if $l < $r { $r } else { $l };
        let b = if $t < $b { $b } else { $t };
        unsafe { $crate::Rect::from_ltrb_unchecked(l, t, r, b) }
    }};
}

/// A 2-dimensional rectangle with integer precision.
///
/// The type parameter `T` is guaranteed to be a built-in Rust integer type, and defaults to `i32`.
///
/// ## Layout
///
/// Each `Rect<T>` is defined by two points, the top-left and bottom-right corners.
///
/// The layout of `Rect<T>` is guaranteed to be the same as a C struct with four fields:
///
/// ```c
/// struct Rect {
///   int l; // x coordinate of the top-left corner
///   int t; // y coordinate of the top-left corner
///   int r; // x coordinate of the bottom-right corner
///   int b; // y coordinate of the bottom-right corner
/// }
/// ```
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct Rect<T: Int = i32> {
    l: T,
    t: T,
    r: T,
    b: T,
}

/// Error type for rectangle operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectError {
    /// The dimensions provided do not form a valid rectangle.
    InvalidDimensions,
}

impl<T: Int> Rect<T> {
    /// An empty rectangle (e.g. a `0x0` region at the origin).
    pub const EMPTY: Self = Self {
        l: T::ZERO,
        t: T::ZERO,
        r: T::ZERO,
        b: T::ZERO,
    };

    /// Creates a new rectangle from the top-left corner and size.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect, Size};
    ///
    /// let rect = Rect::new(Pos::new(1, 2), Size::new(3, 4));
    /// assert_eq!(rect.left(), 1);
    /// assert_eq!(rect.top(), 2);
    /// assert_eq!(rect.right(), 4);
    /// assert_eq!(rect.bottom(), 6);
    /// ```
    pub fn new(top_left: Pos<T>, size: Size) -> Self {
        Self::from_ltwh(top_left.x, top_left.y, size.width, size.height)
    }

    /// Creates a new rectangle from the top-left and bottom-right corners.
    ///
    /// ## Errors
    ///
    /// Returns an error if the top-left corner is not less than the bottom-right corner.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect};
    ///
    /// let rect = Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4));
    /// assert!(rect.is_ok());
    ///
    /// let invalid_rect = Rect::from_tlbr(Pos::new(3, 2), Pos::new(1, 4));
    /// assert!(invalid_rect.is_err());
    /// ```
    pub fn from_tlbr(tl: Pos<T>, br: Pos<T>) -> Result<Self, RectError> {
        if tl.x >= br.x || tl.y >= br.y {
            Err(RectError::InvalidDimensions)
        } else {
            Ok(Self {
                l: tl.x,
                t: tl.y,
                r: br.x,
                b: br.y,
            })
        }
    }

    /// Creates a new rectangle from the `l`eft, `t`op, `r`ight, and `b`ottom coordinates.
    ///
    /// ## Errors
    ///
    /// Returns an error if the provided coordinates do not form a valid rectangle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4);
    /// assert!(rect.is_ok());
    ///
    /// let invalid_rect = Rect::from_ltrb(3, 2, 1, 4);
    /// assert!(invalid_rect.is_err());
    /// ```
    pub fn from_ltrb(l: T, t: T, r: T, b: T) -> Result<Self, RectError> {
        if l > r || t > b {
            Err(RectError::InvalidDimensions)
        } else {
            Ok(Self { l, t, r, b })
        }
    }

    /// Creates a new rectangle from the `l`eft, `t`op, `r`ight, and `b`ottom coordinates.
    ///
    /// ## Safety
    ///
    /// This method does not check if the coordinates form a valid rectangle.
    pub const unsafe fn from_ltrb_unchecked(l: T, t: T, r: T, b: T) -> Self {
        Self { l, t, r, b }
    }

    /// Creates a new rectangle from the `l`eft and `t`op coordinates, and `w`idth and `h`eight.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(1, 2, 3, 4);
    /// assert_eq!(rect.left(), 1);
    /// assert_eq!(rect.top(), 2);
    /// assert_eq!(rect.right(), 4);
    /// assert_eq!(rect.bottom(), 6);
    /// ```
    pub fn from_ltwh(l: T, t: T, w: usize, h: usize) -> Self {
        Self {
            l,
            t,
            r: l + T::from_usize(w),
            b: t + T::from_usize(h),
        }
    }

    /// Returns the top, or y-coordinate of the top edge of the rectangle.
    pub const fn top(&self) -> T {
        self.t
    }

    /// Returns the left, or x-coordinate of the left edge of the rectangle.
    pub const fn left(&self) -> T {
        self.l
    }

    /// Returns the right, or x-coordinate of the right edge of the rectangle.
    pub const fn right(&self) -> T {
        self.r
    }

    /// Returns the bottom, or y-coordinate of the bottom edge of the rectangle.
    pub const fn bottom(&self) -> T {
        self.b
    }

    /// Returns the top-left corner of the rectangle as a [`Pos<T>`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.top_left(), Pos::new(1, 2));
    /// ```
    pub const fn top_left(&self) -> Pos<T> {
        Pos::new(self.l, self.t)
    }

    /// Returns the top-right corner of the rectangle as a [`Pos<T>`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.top_right(), Pos::new(3, 2));
    /// ```
    pub const fn top_right(&self) -> Pos<T> {
        Pos::new(self.r, self.t)
    }

    /// Returns the bottom-right corner of the rectangle as a [`Pos<T>`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.bottom_right(), Pos::new(3, 4));
    /// ```
    pub const fn bottom_right(&self) -> Pos<T> {
        Pos::new(self.r, self.b)
    }

    /// Returns the bottom-left corner of the rectangle as a [`Pos<T>`].
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.bottom_left(), Pos::new(1, 4));
    /// ```
    pub const fn bottom_left(&self) -> Pos<T> {
        Pos::new(self.l, self.b)
    }

    /// Returns the width of the rectangle, which is the distance between the left and right edges.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.width(), 2);
    /// ```
    pub fn width(&self) -> usize {
        (self.r - self.l).to_usize()
    }

    /// Returns the height of the rectangle, which is the distance between the top and bottom edges.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.height(), 2);
    /// ```
    pub fn height(&self) -> usize {
        (self.b - self.t).to_usize()
    }

    /// Returns `true` if the rectangle is empty, i.e., if its width or height is zero.
    pub fn is_empty(&self) -> bool {
        self.width() == 0 || self.height() == 0
    }

    /// Returns the area of the rectangle, which is the product of its width and height.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.area(), 4);
    /// ```
    pub fn area(&self) -> usize {
        self.width() * self.height()
    }

    /// Returns `true` if the rectangle contains the given `x` and `y` coordinates.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert!(rect.contains(2, 3));
    /// assert!(!rect.contains(0, 0));
    /// ```
    pub fn contains(&self, x: T, y: T) -> bool {
        x >= self.l && x < self.r && y >= self.t && y < self.b
    }

    /// Returns `true` if the rectangle contains the given position.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert!(rect.contains_pos(Pos::new(2, 3)));
    /// assert!(!rect.contains_pos(Pos::new(0, 0)));
    /// ```
    pub fn contains_pos(&self, pos: Pos<T>) -> bool {
        self.contains(pos.x, pos.y)
    }

    /// Returns `true` if the rectangle contains the given rectangle.
    ///
    /// If any edge of the given rectangle is outside this rectangle, it returns `false`.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
    /// assert!(rect.contains_rect(Rect::from_ltrb(2, 3, 4, 5).unwrap()));
    ///
    /// assert!(!rect.contains_rect(Rect::from_ltrb(0, 3, 4, 5).unwrap()));
    /// assert!(!rect.contains_rect(Rect::from_ltrb(2, 3, 6, 5).unwrap()));
    /// assert!(!rect.contains_rect(Rect::from_ltrb(2, 3, 4, 7).unwrap()));
    /// ```
    pub fn contains_rect(&self, other: Rect<T>) -> bool {
        self.l <= other.l && self.r >= other.r && self.t <= other.t && self.b >= other.b
    }

    /// Returns the intersection of this rectangle with another rectangle.
    ///
    /// If the rectangles do not overlap, returns an empty rectangle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let a = Rect::from_ltrb(1, 2, 5, 6).unwrap();
    /// let b = Rect::from_ltrb(3, 4, 7, 8).unwrap();
    /// let intersection = a.intersect(b);
    /// assert_eq!(intersection, Rect::from_ltrb(3, 4, 5, 6).unwrap());
    ///
    /// let c = Rect::from_ltrb(6, 7, 8, 9).unwrap();
    /// assert_eq!(a.intersect(c), Rect::EMPTY);
    /// ```
    #[must_use]
    pub fn intersect(&self, other: Rect<T>) -> Rect<T> {
        let l = core::cmp::max(self.l, other.l);
        let t = core::cmp::max(self.t, other.t);
        let r = core::cmp::min(self.r, other.r);
        let b = core::cmp::min(self.b, other.b);

        if l < r && t < b {
            Rect { l, t, r, b }
        } else {
            Rect::EMPTY
        }
    }

    /// Returns an iterator over the positions within the rectangle, in row-major order.
    ///
    /// The positions are exclusive of the bottom-right edge.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// let positions: Vec<Pos<i32>> = rect.into_iter_row_major().collect();
    /// assert_eq!(positions, &[Pos::new(1, 2), Pos::new(2, 2), Pos::new(1, 3), Pos::new(2, 3)]);
    /// ```
    pub fn into_iter_row_major(self) -> impl Iterator<Item = Pos<T>> {
        RowMajor::positions(self)
    }

    /// Returns an iterator over the positions within the rectangle, in column-major order.
    ///
    /// The positions are exclusive of the bottom-right edge.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// let positions: Vec<Pos<i32>> = rect.into_iter_col_major().collect();
    /// assert_eq!(positions, &[Pos::new(1, 2), Pos::new(1, 3), Pos::new(2, 2), Pos::new(2, 3)]);
    /// ```
    pub fn into_iter_col_major(self) -> impl Iterator<Item = Pos<T>> {
        ColMajor::positions(self)
    }
}

impl<T: Int> HasSize for Rect<T> {
    fn size(&self) -> crate::Size {
        crate::Size {
            width: self.width(),
            height: self.height(),
        }
    }
}

impl<T: Int> ops::Add<Pos<T>> for Rect<T> {
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Self {
            l: self.l + rhs.x,
            t: self.t + rhs.y,
            r: self.r + rhs.x,
            b: self.b + rhs.y,
        }
    }
}

impl<T: Int> ops::AddAssign<Pos<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: Pos<T>) {
        self.l += rhs.x;
        self.t += rhs.y;
        self.r += rhs.x;
        self.b += rhs.y;
    }
}

impl<T: Int> ops::Sub<Pos<T>> for Rect<T> {
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Self {
            l: self.l - rhs.x,
            t: self.t - rhs.y,
            r: self.r - rhs.x,
            b: self.b - rhs.y,
        }
    }
}

impl<T: Int> ops::SubAssign<Pos<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: Pos<T>) {
        self.l -= rhs.x;
        self.t -= rhs.y;
        self.r -= rhs.x;
        self.b -= rhs.y;
    }
}

impl<T: Int> ops::Mul<T> for Rect<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            l: self.l * rhs,
            t: self.t * rhs,
            r: self.r * rhs,
            b: self.b * rhs,
        }
    }
}

impl<T: Int> ops::MulAssign<T> for Rect<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.l *= rhs;
        self.t *= rhs;
        self.r *= rhs;
        self.b *= rhs;
    }
}

impl<T: Int> ops::Div<T> for Rect<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            l: self.l / rhs,
            t: self.t / rhs,
            r: self.r / rhs,
            b: self.b / rhs,
        }
    }
}

impl<T: Int> ops::DivAssign<T> for Rect<T> {
    fn div_assign(&mut self, rhs: T) {
        self.l /= rhs;
        self.t /= rhs;
        self.r /= rhs;
        self.b /= rhs;
    }
}

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::vec::Vec;

    #[test]
    fn rect_macro_ltrb() {
        const R: Rect<i32> = rect!(1, 2, 3, 4);
        assert_eq!(R, Rect::from_ltrb(1, 2, 3, 4).unwrap());
    }

    #[test]
    fn rect_macro_ltrb_auto() {
        const R: Rect<i32> = rect!(3, 4, 1, 2);
        assert_eq!(R, Rect::from_ltrb(1, 2, 3, 4).unwrap());
    }

    #[test]
    fn rect_macro_tlbr() {
        const R: Rect<i32> = rect!(Pos::new(1, 2), Pos::new(3, 4));
        assert_eq!(R, Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4)).unwrap());
    }

    #[test]
    fn rect_macro_tlbr_auto() {
        const R: Rect<i32> = rect!(Pos::new(3, 4), Pos::new(1, 2));
        assert_eq!(R, Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4)).unwrap());
    }

    #[test]
    fn from_tlbr_ok() {
        let rect = Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4)).unwrap();
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 3);
        assert_eq!(rect.bottom(), 4);
    }

    #[test]
    fn from_tlbr_err() {
        let rect = Rect::from_tlbr(Pos::new(3, 2), Pos::new(1, 4));
        assert!(rect.is_err());
    }

    #[test]
    fn from_ltrb_ok() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 3);
        assert_eq!(rect.bottom(), 4);
    }

    #[test]
    fn from_ltrb_err() {
        let rect = Rect::from_ltrb(3, 2, 1, 4);
        assert!(rect.is_err());
    }

    #[test]
    fn from_ltwh_ok() {
        let rect = Rect::from_ltwh(1, 2, 3, 4);
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 4);
        assert_eq!(rect.bottom(), 6);
    }

    #[test]
    fn c_layout() {
        struct CRect {
            l: i32,
            t: i32,
            r: i32,
            b: i32,
        }

        let rect = Rect::<i32> {
            l: 1,
            t: 2,
            r: 3,
            b: 4,
        };

        #[allow(unsafe_code, reason = "Test")]
        let c_rect: CRect = unsafe { core::mem::transmute(rect) };
        assert_eq!(c_rect.l, 1);
        assert_eq!(c_rect.t, 2);
        assert_eq!(c_rect.r, 3);
        assert_eq!(c_rect.b, 4);
    }

    #[test]
    fn coords() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 3);
        assert_eq!(rect.bottom(), 4);
    }

    #[test]
    fn corners() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert_eq!(rect.top_left(), Pos::new(1, 2));
        assert_eq!(rect.top_right(), Pos::new(3, 2));
        assert_eq!(rect.bottom_right(), Pos::new(3, 4));
        assert_eq!(rect.bottom_left(), Pos::new(1, 4));
    }

    #[test]
    fn dimensions() {
        let rect = Rect::from_ltrb(1, 2, 3, 6).unwrap();
        assert_eq!(rect.width(), 2);
        assert_eq!(rect.height(), 4);
        assert!(!rect.is_empty());
    }

    #[test]
    fn empty_rect() {
        let rect = Rect::from_ltrb(1, 2, 1, 2).unwrap();
        assert_eq!(rect.width(), 0);
        assert_eq!(rect.height(), 0);
        assert!(rect.is_empty());
    }

    #[test]
    fn area() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert_eq!(rect.area(), 4);
    }

    #[test]
    fn has_size() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert_eq!(
            rect.size(),
            crate::Size {
                width: 2,
                height: 2
            }
        );
    }

    #[test]
    fn contains_pos_true() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert!(rect.contains_pos(Pos::new(2, 3)));
    }

    #[test]
    fn contains_pos_false_x_before_left() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert!(!rect.contains_pos(Pos::new(0, 3)));
    }

    #[test]
    fn contains_pos_false_x_after_right() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert!(!rect.contains_pos(Pos::new(4, 3)));
    }

    #[test]
    fn contains_pos_false_y_before_top() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert!(!rect.contains_pos(Pos::new(2, 1)));
    }

    #[test]
    fn contains_pos_false_y_after_bottom() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        assert!(!rect.contains_pos(Pos::new(2, 5)));
    }

    #[test]
    fn contains_rect_true() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        assert!(rect.contains_rect(Rect::from_ltrb(2, 3, 4, 5).unwrap()));
    }

    #[test]
    fn contains_rect_false_left_edge() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        assert!(!rect.contains_rect(Rect::from_ltrb(0, 3, 4, 5).unwrap()));
    }

    #[test]
    fn contains_rect_false_right_edge() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        assert!(!rect.contains_rect(Rect::from_ltrb(2, 3, 6, 5).unwrap()));
    }

    #[test]
    fn contains_rect_false_top_edge() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        assert!(!rect.contains_rect(Rect::from_ltrb(2, 1, 4, 5).unwrap()));
    }

    #[test]
    fn contains_rect_false_bottom_edge() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        assert!(!rect.contains_rect(Rect::from_ltrb(2, 3, 4, 7).unwrap()));
    }

    #[test]
    fn iter_pos_row_major() {
        let rect = Rect::from_ltrb(0, 0, 3, 2).unwrap();
        let positions: Vec<_> = rect.into_iter_row_major().collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(1, 0),
                Pos::new(2, 0),
                Pos::new(0, 1),
                Pos::new(1, 1),
                Pos::new(2, 1)
            ]
        );
    }

    #[test]
    fn iter_pos_col_major() {
        let rect = Rect::from_ltrb(0, 0, 3, 2).unwrap();
        let positions: Vec<_> = rect.into_iter_col_major().collect();
        assert_eq!(
            positions,
            &[
                Pos::new(0, 0),
                Pos::new(0, 1),
                Pos::new(1, 0),
                Pos::new(1, 1),
                Pos::new(2, 0),
                Pos::new(2, 1)
            ]
        );
    }

    #[test]
    fn iter_pos_row_major_offset() {
        let rect = Rect::from_ltrb(1, 2, 3, 6).unwrap();
        let positions: Vec<_> = rect.into_iter_row_major().collect();
        assert_eq!(
            positions,
            &[
                Pos::new(1, 2),
                Pos::new(2, 2),
                Pos::new(1, 3),
                Pos::new(2, 3),
                Pos::new(1, 4),
                Pos::new(2, 4),
                Pos::new(1, 5),
                Pos::new(2, 5)
            ]
        );
    }

    #[test]
    fn intersect_full() {
        let a = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let b = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let intersection = a.intersect(b);
        assert_eq!(intersection, a);
    }

    #[test]
    fn intersect_partial() {
        let a = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let b = Rect::from_ltrb(3, 4, 7, 8).unwrap();
        let intersection = a.intersect(b);
        assert_eq!(intersection, Rect::from_ltrb(3, 4, 5, 6).unwrap());
    }

    #[test]
    fn intersect_none() {
        let a = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let b = Rect::from_ltrb(6, 7, 8, 9).unwrap();
        let intersection = a.intersect(b);
        assert_eq!(intersection, Rect::EMPTY);
    }

    #[test]
    fn from_ltrb_unchecked() {
        let rect = unsafe { Rect::from_ltrb_unchecked(1, 2, 3, 4) };
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 3);
        assert_eq!(rect.bottom(), 4);
    }

    #[test]
    fn add_pos() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let pos = Pos::new(1, 1);
        let new_rect = rect + pos;
        assert_eq!(new_rect.left(), 2);
        assert_eq!(new_rect.top(), 3);
        assert_eq!(new_rect.right(), 4);
        assert_eq!(new_rect.bottom(), 5);
    }

    #[test]
    fn add_assign_pos() {
        let mut rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let pos = Pos::new(1, 1);
        rect += pos;
        assert_eq!(rect.left(), 2);
        assert_eq!(rect.top(), 3);
        assert_eq!(rect.right(), 4);
        assert_eq!(rect.bottom(), 5);
    }

    #[test]
    fn sub_pos() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let pos = Pos::new(1, 1);
        let new_rect = rect - pos;
        assert_eq!(new_rect.left(), 0);
        assert_eq!(new_rect.top(), 1);
        assert_eq!(new_rect.right(), 2);
        assert_eq!(new_rect.bottom(), 3);
    }

    #[test]
    fn sub_assign_pos() {
        let mut rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let pos = Pos::new(1, 1);
        rect -= pos;
        assert_eq!(rect.left(), 0);
        assert_eq!(rect.top(), 1);
        assert_eq!(rect.right(), 2);
        assert_eq!(rect.bottom(), 3);
    }

    #[test]
    fn mul_int() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let new_rect = rect * 2;
        assert_eq!(new_rect.left(), 2);
        assert_eq!(new_rect.top(), 4);
        assert_eq!(new_rect.right(), 6);
        assert_eq!(new_rect.bottom(), 8);
    }

    #[test]
    fn mul_assign_int() {
        let mut rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        rect *= 2;
        assert_eq!(rect.left(), 2);
        assert_eq!(rect.top(), 4);
        assert_eq!(rect.right(), 6);
        assert_eq!(rect.bottom(), 8);
    }

    #[test]
    fn div_int() {
        let rect = Rect::from_ltrb(2, 4, 6, 8).unwrap();
        let new_rect = rect / 2;
        assert_eq!(new_rect.left(), 1);
        assert_eq!(new_rect.top(), 2);
        assert_eq!(new_rect.right(), 3);
        assert_eq!(new_rect.bottom(), 4);
    }

    #[test]
    fn div_assign_int() {
        let mut rect = Rect::from_ltrb(2, 4, 6, 8).unwrap();
        rect /= 2;
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 3);
        assert_eq!(rect.bottom(), 4);
    }
}
