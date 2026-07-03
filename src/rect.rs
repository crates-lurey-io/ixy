use core::{fmt::Display, ops};

use crate::{
    HasSize, Pos, Size,
    int::Int,
    layout::{Layout, RowMajor},
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
        $crate::Rect::from_ltrb_unchecked(l, t, r, b)
    }};
    ($l:expr, $t:expr, $r:expr, $b:expr) => {{
        let l = if $l < $r { $l } else { $r };
        let t = if $t < $b { $t } else { $b };
        let r = if $l < $r { $r } else { $l };
        let b = if $t < $b { $b } else { $t };
        $crate::Rect::from_ltrb_unchecked(l, t, r, b)
    }};
}

/// A 2-dimensional rectangle with integer precision.
///
/// The type parameter `T` is guaranteed to be a built-in Rust integer type, and defaults to `i32`.
///
/// ## Layout
///
/// Each `Rect<T>` is defined as an origin point `(x, y)` and dimensions `(w, h)`.
///
/// The layout of `Rect<T>` is guaranteed to be the same as a C struct with four fields:
///
/// ```c
/// struct Rect {
///   int x;
///   int y;
///   int w;
///   int h;
/// }
/// ```
#[repr(C)]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Rect<T = i32> {
    x: T,
    y: T,
    w: T,
    h: T,
}

/// Error type for rectangle operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RectError {
    /// The dimensions provided do not form a valid rectangle.
    InvalidDimensions,
}

impl Display for RectError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::InvalidDimensions => {
                write!(f, "the provided coordinates do not form a valid rectangle")
            }
        }
    }
}

impl core::error::Error for RectError {}

impl<T: Int> Rect<T> {
    /// An empty rectangle (e.g. a `0x0` region at the origin).
    pub const EMPTY: Self = Self {
        x: T::ZERO,
        y: T::ZERO,
        w: T::ZERO,
        h: T::ZERO,
    };

    /// Creates a rectangle from top-left coordinates and dimensions.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::new(1, 2, 3, 4);
    /// assert_eq!(rect.left(), 1);
    /// assert_eq!(rect.top(), 2);
    /// assert_eq!(rect.right(), 4);
    /// assert_eq!(rect.bottom(), 6);
    /// ```
    #[must_use]
    pub const fn new(x: T, y: T, width: T, height: T) -> Self {
        Self::from_ltwh(x, y, width, height)
    }

    /// Creates a rectangle from a top-left corner position and size.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect, Size};
    ///
    /// let rect = Rect::from_tl_size(Pos::new(1, 2), Size::new(3, 4));
    /// assert_eq!(rect.left(), 1);
    /// assert_eq!(rect.top(), 2);
    /// assert_eq!(rect.right(), 4);
    /// assert_eq!(rect.bottom(), 6);
    /// ```
    #[must_use]
    pub const fn from_tl_size(top_left: Pos<T>, size: Size<T>) -> Self {
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
                x: tl.x,
                y: tl.y,
                w: br.x - tl.x,
                h: br.y - tl.y,
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
            Ok(Self {
                x: l,
                y: t,
                w: r - l,
                h: b - t,
            })
        }
    }

    /// Creates a new rectangle from the `l`eft, `t`op, `r`ight, and `b`ottom coordinates.
    ///
    /// The caller must ensure `l <= r` and `t <= b`; in debug builds this is checked.
    #[must_use]
    pub fn from_ltrb_unchecked(l: T, t: T, r: T, b: T) -> Self {
        debug_assert!(l <= r && t <= b);
        Self {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
        }
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
    #[must_use]
    pub const fn from_ltwh(l: T, t: T, w: T, h: T) -> Self {
        Self { x: l, y: t, w, h }
    }

    /// Returns the top, or y-coordinate of the top edge of the rectangle.
    #[must_use]
    pub const fn top(&self) -> T {
        self.y
    }

    /// Returns the left, or x-coordinate of the left edge of the rectangle.
    #[must_use]
    pub const fn left(&self) -> T {
        self.x
    }

    /// Returns the right, or x-coordinate of the right edge of the rectangle.
    #[must_use]
    pub fn right(&self) -> T {
        self.x + self.w
    }

    /// Returns the bottom, or y-coordinate of the bottom edge of the rectangle.
    #[must_use]
    pub fn bottom(&self) -> T {
        self.y + self.h
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
    #[must_use]
    pub const fn top_left(&self) -> Pos<T> {
        Pos::new(self.x, self.y)
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
    #[must_use]
    pub fn top_right(&self) -> Pos<T> {
        Pos::new(self.x + self.w, self.y)
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
    #[must_use]
    pub fn bottom_right(&self) -> Pos<T> {
        Pos::new(self.x + self.w, self.y + self.h)
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
    #[must_use]
    pub fn bottom_left(&self) -> Pos<T> {
        Pos::new(self.x, self.y + self.h)
    }

    /// Returns the width of the rectangle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.width(), 2);
    /// ```
    #[must_use]
    pub const fn width(&self) -> T {
        self.w
    }

    /// Returns the height of the rectangle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.height(), 2);
    /// ```
    #[must_use]
    pub const fn height(&self) -> T {
        self.h
    }

    /// Returns the width of the rectangle as a [`usize`], for use in indexing.
    #[must_use]
    pub fn width_usize(&self) -> usize {
        self.w.to_usize()
    }

    /// Returns the height of the rectangle as a [`usize`], for use in indexing.
    #[must_use]
    pub fn height_usize(&self) -> usize {
        self.h.to_usize()
    }

    /// Returns `true` if the rectangle is empty, i.e., if its width or height is zero.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.w == T::ZERO || self.h == T::ZERO
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
    #[must_use]
    pub fn area(&self) -> usize {
        self.width_usize() * self.height_usize()
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
    #[must_use]
    pub fn contains(&self, x: T, y: T) -> bool {
        let r = self.x + self.w;
        let b = self.y + self.h;
        x >= self.x && x < r && y >= self.y && y < b
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
    #[must_use]
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
    #[must_use]
    pub fn contains_rect(&self, other: Self) -> bool {
        let sr = self.x + self.w;
        let sb = self.y + self.h;
        let or = other.x + other.w;
        let ob = other.y + other.h;
        self.x <= other.x && sr >= or && self.y <= other.y && sb >= ob
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
    pub fn intersect(&self, other: Self) -> Self {
        let sr = self.x + self.w;
        let sb = self.y + self.h;
        let or = other.x + other.w;
        let ob = other.y + other.h;

        let l = core::cmp::max(self.x, other.x);
        let t = core::cmp::max(self.y, other.y);
        let r = core::cmp::min(sr, or);
        let b = core::cmp::min(sb, ob);

        if l < r && t < b {
            Self {
                x: l,
                y: t,
                w: r - l,
                h: b - t,
            }
        } else {
            Self::EMPTY
        }
    }

    /// Returns an iterator over the positions in the rectangle.
    ///
    /// The positions are returned in row-major order, starting from the top-left corner.
    ///
    /// For additional traversal methods, see the [`layout`][] module.
    ///
    /// [`layout`]: crate::layout
    pub fn pos_iter(&self) -> impl Iterator<Item = Pos<T>> {
        RowMajor::iter_pos(*self)
    }

    /// Returns a sub-rectangle representing a row within this rectangle.
    ///
    /// The returned rectangle is guaranteed to be within the bounds of this rectangle.
    #[must_use]
    pub fn row_rect(&self, row: usize) -> Self {
        Self {
            x: self.x,
            y: self.y + T::from_usize(row),
            w: self.w,
            h: T::ONE,
        }
    }

    /// Returns a sub-rectangle representing a column within this rectangle.
    ///
    /// The returned rectangle is guaranteed to be within the bounds of this rectangle.
    #[must_use]
    pub fn col_rect(&self, col: usize) -> Self {
        Self {
            x: self.x + T::from_usize(col),
            y: self.y,
            w: T::ONE,
            h: self.h,
        }
    }

    /// Returns the smallest rectangle that contains both `self` and `other`.
    ///
    /// If either rectangle is empty, the other rectangle is returned unchanged.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
    /// let b = Rect::from_ltrb(1, 1, 4, 4).unwrap();
    /// assert_eq!(a.union(b), Rect::from_ltrb(0, 0, 4, 4).unwrap());
    /// ```
    #[must_use]
    pub fn union(&self, other: Self) -> Self {
        if self.is_empty() {
            return other;
        }
        if other.is_empty() {
            return *self;
        }
        let l = core::cmp::min(self.x, other.x);
        let t = core::cmp::min(self.y, other.y);
        let r = core::cmp::max(self.right(), other.right());
        let b = core::cmp::max(self.bottom(), other.bottom());
        Self {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
        }
    }

    /// Returns a rectangle grown by `dx` on the left/right edges and `dy` on the top/bottom
    /// edges.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(2, 2, 4, 4);
    /// assert_eq!(rect.inflate(1, 1), Rect::from_ltwh(1, 1, 6, 6));
    /// ```
    #[must_use]
    pub fn inflate(&self, dx: T, dy: T) -> Self {
        Self {
            x: self.x - dx,
            y: self.y - dy,
            w: self.w + dx + dx,
            h: self.h + dy + dy,
        }
    }

    /// Returns a rectangle shrunk by `dx` on the left/right edges and `dy` on the top/bottom
    /// edges.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(1, 1, 6, 6);
    /// assert_eq!(rect.shrink(1, 1), Rect::from_ltwh(2, 2, 4, 4));
    /// ```
    #[must_use]
    pub fn shrink(&self, dx: T, dy: T) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w - dx - dx,
            h: self.h - dy - dy,
        }
    }

    /// Returns the center point of the rectangle.
    ///
    /// Integer division rounds the result towards the top-left.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// assert_eq!(rect.center(), Pos::new(2, 2));
    /// ```
    #[must_use]
    pub fn center(&self) -> Pos<T> {
        let two = T::ONE + T::ONE;
        Pos::new(self.x + self.w / two, self.y + self.h / two)
    }

    /// Returns `true` if this rectangle overlaps with `other`, i.e. their intersection is
    /// non-empty.
    ///
    /// Unlike [`Rect::intersect`], this does not construct the overlapping rectangle.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
    /// let b = Rect::from_ltrb(1, 1, 3, 3).unwrap();
    /// let c = Rect::from_ltrb(2, 2, 4, 4).unwrap();
    /// assert!(a.overlaps(b));
    /// assert!(!a.overlaps(c));
    /// ```
    #[must_use]
    pub fn overlaps(&self, other: Self) -> bool {
        self.x < other.right()
            && other.x < self.right()
            && self.y < other.bottom()
            && other.y < self.bottom()
    }
}

/// Iterator over the positions in a [`Rect<T>`], in row-major order.
///
/// Returned by [`Rect::into_iter`].
pub struct IntoIter<T: Int> {
    current: Pos<T>,
    bounds: Rect<T>,
}

impl<T: Int> Iterator for IntoIter<T> {
    type Item = Pos<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current.y >= self.bounds.bottom() {
            return None;
        }
        let pos = self.current;
        self.current.x += T::ONE;
        if self.current.x >= self.bounds.right() {
            self.current.x = self.bounds.left();
            self.current.y += T::ONE;
        }
        Some(pos)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for IntoIter<T> {
    fn len(&self) -> usize {
        if self.current.y >= self.bounds.bottom() {
            return 0;
        }
        let width = (self.bounds.right() - self.bounds.left()).to_usize();
        let remaining_in_row = (self.bounds.right() - self.current.x).to_usize();
        let remaining_rows = (self.bounds.bottom() - self.current.y).to_usize() - 1;
        remaining_in_row + remaining_rows * width
    }
}

impl<T: Int> core::iter::FusedIterator for IntoIter<T> {}

impl<T: Int> IntoIterator for Rect<T> {
    type Item = Pos<T>;
    type IntoIter = IntoIter<T>;

    /// Returns an iterator over the positions in the rectangle, in row-major order.
    ///
    /// Equivalent to [`Rect::pos_iter`], but usable in `for pos in rect` syntax.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Pos, Rect};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 2, 1);
    /// let mut positions = vec![];
    /// for pos in rect {
    ///     positions.push(pos);
    /// }
    /// assert_eq!(positions, &[Pos::new(0, 0), Pos::new(1, 0)]);
    /// ```
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            current: self.top_left(),
            bounds: self,
        }
    }
}

impl<T: Display + Int> Display for Rect<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Rect({}, {}, {}, {})", self.x, self.y, self.w, self.h)
    }
}

impl<T: Int> HasSize<T> for Rect<T> {
    fn size(&self) -> Size<T> {
        Size {
            width: self.w,
            height: self.h,
        }
    }
}

impl<T: Int> ops::Add<Pos<T>> for Rect<T> {
    type Output = Self;

    fn add(self, rhs: Pos<T>) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl<T: Int> ops::AddAssign<Pos<T>> for Rect<T> {
    fn add_assign(&mut self, rhs: Pos<T>) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl<T: Int> ops::Sub<Pos<T>> for Rect<T> {
    type Output = Self;

    fn sub(self, rhs: Pos<T>) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            w: self.w,
            h: self.h,
        }
    }
}

impl<T: Int> ops::SubAssign<Pos<T>> for Rect<T> {
    fn sub_assign(&mut self, rhs: Pos<T>) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<T: Int> ops::Mul<T> for Rect<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Self {
            x: self.x * rhs,
            y: self.y * rhs,
            w: self.w * rhs,
            h: self.h * rhs,
        }
    }
}

impl<T: Int> ops::MulAssign<T> for Rect<T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x *= rhs;
        self.y *= rhs;
        self.w *= rhs;
        self.h *= rhs;
    }
}

impl<T: Int> ops::Div<T> for Rect<T> {
    type Output = Self;

    fn div(self, rhs: T) -> Self::Output {
        Self {
            x: self.x / rhs,
            y: self.y / rhs,
            w: self.w / rhs,
            h: self.h / rhs,
        }
    }
}

impl<T: Int> ops::DivAssign<T> for Rect<T> {
    fn div_assign(&mut self, rhs: T) {
        self.x /= rhs;
        self.y /= rhs;
        self.w /= rhs;
        self.h /= rhs;
    }
}

/// A rectangle using `u16` coordinates.
pub type Rect16 = Rect<u16>;

/// A rectangle using `i32` coordinates — the default.
pub type RectI = Rect<i32>;

#[cfg(test)]
mod tests {
    extern crate alloc;

    use super::*;
    use alloc::{string::ToString, vec::Vec};

    #[test]
    fn rect_error_display() {
        assert_eq!(
            RectError::InvalidDimensions.to_string(),
            "the provided coordinates do not form a valid rectangle"
        );
    }

    #[test]
    fn rect_error_is_error() {
        fn assert_error<E: core::error::Error>(_: &E) {}
        assert_error(&RectError::InvalidDimensions);
    }

    #[test]
    fn rect_macro_ltrb() {
        let r: Rect<i32> = rect!(1, 2, 3, 4);
        assert_eq!(r, Rect::from_ltrb(1, 2, 3, 4).unwrap());
    }

    #[test]
    fn rect_macro_ltrb_auto() {
        let r: Rect<i32> = rect!(3, 4, 1, 2);
        assert_eq!(r, Rect::from_ltrb(1, 2, 3, 4).unwrap());
    }

    #[test]
    fn rect_macro_tlbr() {
        let r: Rect<i32> = rect!(Pos::new(1, 2), Pos::new(3, 4));
        assert_eq!(r, Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4)).unwrap());
    }

    #[test]
    fn rect_macro_tlbr_auto() {
        let r: Rect<i32> = rect!(Pos::new(3, 4), Pos::new(1, 2));
        assert_eq!(r, Rect::from_tlbr(Pos::new(1, 2), Pos::new(3, 4)).unwrap());
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
    fn new_xywh() {
        let rect = Rect::new(1, 2, 3, 4);
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 4);
        assert_eq!(rect.bottom(), 6);
    }

    #[test]
    fn new_xywh_zero() {
        let rect = Rect::new(0, 0, 0, 0);
        assert!(rect.is_empty());
        assert_eq!(rect.left(), 0);
        assert_eq!(rect.top(), 0);
        assert_eq!(rect.right(), 0);
        assert_eq!(rect.bottom(), 0);
    }

    #[test]
    fn from_tl_size() {
        use crate::Size;
        let rect = Rect::from_tl_size(Pos::new(1, 2), Size::new(3, 4));
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 4);
        assert_eq!(rect.bottom(), 6);
    }

    #[test]
    fn rect16_alias() {
        let rect: Rect16 = Rect16::new(1, 2, 3, 4);
        assert_eq!(rect.left(), 1u16);
        assert_eq!(rect.top(), 2u16);
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 4);
    }

    #[test]
    fn recti_alias() {
        let rect: RectI = RectI::new(1, 2, 3, 4);
        assert_eq!(rect.left(), 1i32);
        assert_eq!(rect.top(), 2i32);
        assert_eq!(rect.width(), 3);
        assert_eq!(rect.height(), 4);
    }

    #[test]
    fn c_layout() {
        use core::mem::{offset_of, size_of};

        #[repr(C)]
        struct CRect {
            x: i32,
            y: i32,
            w: i32,
            h: i32,
        }

        assert_eq!(size_of::<Rect<i32>>(), size_of::<CRect>());
        assert_eq!(offset_of!(Rect<i32>, x), offset_of!(CRect, x));
        assert_eq!(offset_of!(Rect<i32>, y), offset_of!(CRect, y));
        assert_eq!(offset_of!(Rect<i32>, w), offset_of!(CRect, w));
        assert_eq!(offset_of!(Rect<i32>, h), offset_of!(CRect, h));
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
            Size {
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
    fn union_overlapping() {
        let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        let b = Rect::from_ltrb(1, 1, 4, 4).unwrap();
        assert_eq!(a.union(b), Rect::from_ltrb(0, 0, 4, 4).unwrap());
    }

    #[test]
    fn union_disjoint() {
        let a = Rect::from_ltrb(0, 0, 1, 1).unwrap();
        let b = Rect::from_ltrb(5, 5, 6, 6).unwrap();
        assert_eq!(a.union(b), Rect::from_ltrb(0, 0, 6, 6).unwrap());
    }

    #[test]
    fn union_with_empty() {
        let a = Rect::from_ltrb(1, 1, 3, 3).unwrap();
        assert_eq!(a.union(Rect::EMPTY), a);
        assert_eq!(Rect::EMPTY.union(a), a);
    }

    #[test]
    fn inflate() {
        let rect = Rect::from_ltwh(2, 2, 4, 4);
        assert_eq!(rect.inflate(1, 1), Rect::from_ltwh(1, 1, 6, 6));
    }

    #[test]
    fn shrink() {
        let rect = Rect::from_ltwh(1, 1, 6, 6);
        assert_eq!(rect.shrink(1, 1), Rect::from_ltwh(2, 2, 4, 4));
    }

    #[test]
    fn inflate_then_shrink_is_identity() {
        let rect = Rect::from_ltwh(3, 3, 5, 5);
        assert_eq!(rect.inflate(2, 2).shrink(2, 2), rect);
    }

    #[test]
    fn center_even() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(rect.center(), Pos::new(2, 2));
    }

    #[test]
    fn center_odd_rounds_towards_top_left() {
        let rect = Rect::from_ltwh(0, 0, 5, 5);
        assert_eq!(rect.center(), Pos::new(2, 2));
    }

    #[test]
    fn overlaps_true() {
        let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        let b = Rect::from_ltrb(1, 1, 3, 3).unwrap();
        assert!(a.overlaps(b));
    }

    #[test]
    fn overlaps_false_adjacent() {
        let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        let b = Rect::from_ltrb(2, 2, 4, 4).unwrap();
        assert!(!a.overlaps(b));
    }

    #[test]
    fn overlaps_false_disjoint() {
        let a = Rect::from_ltrb(0, 0, 2, 2).unwrap();
        let b = Rect::from_ltrb(5, 5, 6, 6).unwrap();
        assert!(!a.overlaps(b));
    }

    #[test]
    fn into_iter_matches_pos_iter() {
        let rect = Rect::from_ltwh(0, 0, 2, 2);
        let via_pos_iter: Vec<Pos<i32>> = rect.pos_iter().collect();
        let via_into_iter: Vec<Pos<i32>> = rect.into_iter().collect();
        assert_eq!(via_pos_iter, via_into_iter);
    }

    #[test]
    fn into_iter_for_loop() {
        let rect = Rect::from_ltwh(0, 0, 2, 1);
        let mut positions = Vec::new();
        for pos in rect {
            positions.push(pos);
        }
        assert_eq!(positions, &[Pos::new(0, 0), Pos::new(1, 0)]);
    }

    #[test]
    fn into_iter_len() {
        let rect = Rect::from_ltwh(0, 0, 3, 3);
        let mut iter = rect.into_iter();
        assert_eq!(iter.len(), 9);
        iter.next();
        iter.next();
        iter.next();
        iter.next();
        assert_eq!(iter.len(), 5);
        assert_eq!(iter.count(), 5);
    }

    #[test]
    fn from_ltrb_unchecked() {
        let rect = Rect::from_ltrb_unchecked(1, 2, 3, 4);
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

    #[test]
    fn pos_iter() {
        let rect = Rect::from_ltrb(1, 2, 3, 4).unwrap();
        let positions: Vec<Pos<i32>> = rect.pos_iter().collect();
        assert_eq!(
            positions,
            &[
                Pos::new(1, 2),
                Pos::new(2, 2),
                Pos::new(1, 3),
                Pos::new(2, 3)
            ]
        );
    }

    #[test]
    fn row_rect() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let row_rect = rect.row_rect(0);
        assert_eq!(row_rect.left(), 1);
        assert_eq!(row_rect.top(), 2);
        assert_eq!(row_rect.right(), 5);
        assert_eq!(row_rect.bottom(), 3);
    }

    #[test]
    fn col_rect() {
        let rect = Rect::from_ltrb(1, 2, 5, 6).unwrap();
        let col_rect = rect.col_rect(0);
        assert_eq!(col_rect.left(), 1);
        assert_eq!(col_rect.top(), 2);
        assert_eq!(col_rect.right(), 2);
        assert_eq!(col_rect.bottom(), 6);
    }
}
