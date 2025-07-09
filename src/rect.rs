use crate::{
    Pos,
    int::{Int, UnsignedInt},
};

/// A macro that creates a rectangle with the given coordinates.
///
/// Unlike [`Rect::from_tlbr`] or [`Rect::from_ltrb`], this macro is infallible as it guarantees
/// that the coordinates form a valid rectangle, by re-arranging them if necessary; i.e. swapping
/// either the left and right coordinates, or the top and bottom coordinates.
///
/// # Examples
///
/// ```rust
/// use ixy::{Rect, Pos};
///
/// let rect_ltrb = rect!(1, 2, 3, 4);
/// let rect_tlbr = rect!(Pos::new(1, 2), Pos::new(3, 4));
/// ```
#[macro_export]
macro_rules! rect {
    ($tl: expr, $br: expr) => {
        Rect {
            l: if $tl.x < $br.x { $tl.x } else { $br.x },
            t: if $tl.y < $br.y { $tl.y } else { $br.y },
            r: if $tl.x < $br.x { $br.x } else { $tl.x },
            b: if $tl.y < $br.y { $br.y } else { $tl.y },
        }
    };
    ($l:expr, $t:expr, $r:expr, $b:expr) => {
        Rect {
            l: if $l < $r { $l } else { $r },
            t: if $t < $b { $t } else { $b },
            r: if $l < $r { $r } else { $l },
            b: if $t < $b { $b } else { $t },
        }
    };
}

/// A 2-dimensional rectangle with integer precision.
///
/// The type parameter `T` is guaranteed to be a built-in Rust integer type, and defaults to `i32`.
///
/// # Layout
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    /// Creates a new rectangle from the top-left and bottom-right corners.
    ///
    /// # Errors
    ///
    /// Returns an error if the top-left corner is not less than the bottom-right corner.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
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
    /// # Errors
    ///
    /// Returns an error if the provided coordinates do not form a valid rectangle.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4);
    /// assert!(rect.is_ok());
    ///
    /// let invalid_rect = Rect::with_ltrb(3, 2, 1, 4);
    /// assert!(invalid_rect.is_err());
    /// ```
    pub fn from_ltrb(l: T, t: T, r: T, b: T) -> Result<Self, RectError> {
        if l > r || t > b {
            Err(RectError::InvalidDimensions)
        } else {
            Ok(Self { l, t, r, b })
        }
    }

    /// Creates a new rectangle from the `l`eft and `t`op coordinates, and `w`idth and `h`eight.
    ///
    /// # Errors
    ///
    /// Returns an error if either the width or height is negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(1, 2, 3, 4);
    /// assert!(rect.is_ok());
    ///
    /// let invalid_rect = Rect::from_ltwh(1, 2, -3, 4);
    /// assert!(invalid_rect.is_err());
    /// ```
    pub fn from_ltwh(l: T, t: T, w: T, h: T) -> Result<Self, RectError> {
        if w < T::ZERO || h < T::ZERO {
            Err(RectError::InvalidDimensions)
        } else {
            Ok(Self {
                l,
                t,
                r: l + w,
                b: t + h,
            })
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
    /// # Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.top_left(), Pos::new(1, 2));
    /// ```
    pub const fn top_left(&self) -> Pos<T> {
        Pos::new(self.l, self.t)
    }

    /// Returns the top-right corner of the rectangle as a [`Pos<T>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.top_right(), Pos::new(3, 2));
    /// ```
    pub const fn top_right(&self) -> Pos<T> {
        Pos::new(self.r, self.t)
    }

    /// Returns the bottom-right corner of the rectangle as a [`Pos<T>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.bottom_right(), Pos::new(3, 4));
    /// ```
    pub const fn bottom_right(&self) -> Pos<T> {
        Pos::new(self.r, self.b)
    }

    /// Returns the bottom-left corner of the rectangle as a [`Pos<T>`].
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::{Rect, Pos};
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.bottom_left(), Pos::new(1, 4));
    /// ```
    pub const fn bottom_left(&self) -> Pos<T> {
        Pos::new(self.l, self.b)
    }

    /// Returns the width of the rectangle, which is the distance between the left and right edges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.width(), 2);
    /// ```
    pub fn width(&self) -> T {
        self.r - self.l
    }

    /// Returns the height of the rectangle, which is the distance between the top and bottom edges.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.height(), 2);
    /// ```
    pub fn height(&self) -> T {
        self.b - self.t
    }

    /// Returns `true` if the rectangle is empty, i.e., if its width or height is zero.
    pub fn is_empty(&self) -> bool {
        self.width() == T::ZERO || self.height() == T::ZERO
    }

    /// Returns the area of the rectangle, which is the product of its width and height.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::with_ltrb(1, 2, 3, 4).unwrap();
    /// assert_eq!(rect.area(), 4);
    /// ```
    pub fn area(&self) -> T {
        self.width() * self.height()
    }
}

impl<T: UnsignedInt> Rect<T> {
    /// Creates a new rectangle from the left, top, width, and height.
    ///
    /// Unlike [`Rect::from_ltwh`], this method is infallible as `T` is always non-negative.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::<u32>::from_ltwh_unsigned(1, 2, 3, 4);
    /// assert_eq!(rect.left(), 1);
    /// assert_eq!(rect.top(), 2);
    /// assert_eq!(rect.right(), 4);
    /// assert_eq!(rect.bottom(), 6);
    /// ```
    pub fn from_ltwh_unsigned(l: T, t: T, w: T, h: T) -> Rect<T> {
        Rect {
            l,
            t,
            r: l + w,
            b: t + h,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        let rect = Rect::from_ltwh(1, 2, 3, 4).unwrap();
        assert_eq!(rect.left(), 1);
        assert_eq!(rect.top(), 2);
        assert_eq!(rect.right(), 4);
        assert_eq!(rect.bottom(), 6);
    }

    #[test]
    fn from_ltwh_err() {
        let rect = Rect::from_ltwh(1, 2, -3, 4);
        assert!(rect.is_err());
    }

    #[test]
    fn from_ltwh_unsigned() {
        let rect = Rect::<u32>::from_ltwh_unsigned(1, 2, 3, 4);
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
}
