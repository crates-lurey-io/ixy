use core::{fmt::Display, ops};

use crate::{
    HasSize, Pos, Size,
    int::Int,
    layout::{ColumnMajor, Layout, RowMajor},
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
    ///
    /// Saturates at `T::MAX` instead of overflowing/panicking if `left() + width()` would exceed
    /// the range of `T`.
    #[must_use]
    pub fn right(&self) -> T {
        self.x.saturating_add(self.w)
    }

    /// Returns the bottom, or y-coordinate of the bottom edge of the rectangle.
    ///
    /// Saturates at `T::MAX` instead of overflowing/panicking if `top() + height()` would exceed
    /// the range of `T`.
    #[must_use]
    pub fn bottom(&self) -> T {
        self.y.saturating_add(self.h)
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
        Pos::new(self.right(), self.y)
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
        Pos::new(self.right(), self.bottom())
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
        Pos::new(self.x, self.bottom())
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
        x >= self.x && x < self.right() && y >= self.y && y < self.bottom()
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
        self.x <= other.x
            && self.right() >= other.right()
            && self.y <= other.y
            && self.bottom() >= other.bottom()
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
        let l = core::cmp::max(self.x, other.x);
        let t = core::cmp::max(self.y, other.y);
        let r = core::cmp::min(self.right(), other.right());
        let b = core::cmp::min(self.bottom(), other.bottom());

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
    pub fn pos_iter(&self) -> impl Iterator<Item = Pos<T>> + use<T> {
        RowMajor::iter_pos(*self)
    }

    /// Returns a sub-rectangle representing a row within this rectangle.
    ///
    /// The returned rectangle is guaranteed to be within the bounds of this rectangle: `row` is
    /// clamped (saturating) to the last valid row if it would otherwise land outside.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// assert_eq!(rect.row_rect(1), Rect::from_ltwh(0, 1, 4, 1));
    ///
    /// // Out-of-bounds rows clamp to the last valid row instead of escaping the rectangle.
    /// assert_eq!(rect.row_rect(10), rect.row_rect(3));
    /// ```
    #[must_use]
    pub fn row_rect(&self, row: usize) -> Self {
        let row = row.min(self.height_usize().saturating_sub(1));
        Self {
            x: self.x,
            y: self.y + T::from_usize(row),
            w: self.w,
            h: T::ONE,
        }
    }

    /// Returns a sub-rectangle representing a column within this rectangle.
    ///
    /// The returned rectangle is guaranteed to be within the bounds of this rectangle: `col` is
    /// clamped (saturating) to the last valid column if it would otherwise land outside.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(0, 0, 4, 4);
    /// assert_eq!(rect.col_rect(1), Rect::from_ltwh(1, 0, 1, 4));
    ///
    /// // Out-of-bounds columns clamp to the last valid column instead of escaping the rectangle.
    /// assert_eq!(rect.col_rect(10), rect.col_rect(3));
    /// ```
    #[must_use]
    pub fn col_rect(&self, col: usize) -> Self {
        let col = col.min(self.width_usize().saturating_sub(1));
        Self {
            x: self.x + T::from_usize(col),
            y: self.y,
            w: T::ONE,
            h: self.h,
        }
    }

    /// The single-row rects of `self`, top to bottom. Empty if `self` is empty.
    ///
    /// This is the iterator form of [`Rect::row_rect`]: `rect.rows().nth(i) == Some(rect.row_rect(i))`
    /// for any in-range `i`. Unlike [`Rect::row_rect`], which clamps out-of-range indices to the
    /// last valid row, `rows()` simply stops after `self.height()` rects instead of clamping.
    ///
    /// An empty rectangle (zero width *or* zero height, per [`Rect::is_empty`]) yields no rows at
    /// all, even when its height is nonzero: a rect with `width() == 0` has no columns to give a
    /// row any content, and the row rects that would otherwise be produced are themselves empty,
    /// so `rows()` treats such a rectangle the same as a `0`-height one.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(0, 0, 3, 2);
    /// let rows: Vec<_> = rect.rows().collect();
    /// assert_eq!(rows, &[Rect::from_ltwh(0, 0, 3, 1), Rect::from_ltwh(0, 1, 3, 1)]);
    ///
    /// // The motivating case: building per-row output by iterating rows.
    /// let mut lines = Vec::new();
    /// for row in rect.rows() {
    ///     lines.push(format!("row at y={} spans {} cells", row.top(), row.width()));
    /// }
    /// assert_eq!(lines, &["row at y=0 spans 3 cells", "row at y=1 spans 3 cells"]);
    ///
    /// // A zero-width rect yields no rows, even though its height is nonzero.
    /// let empty = Rect::from_ltwh(0, 0, 0, 3);
    /// assert_eq!(empty.rows().count(), 0);
    /// ```
    #[must_use]
    pub fn rows(&self) -> impl ExactSizeIterator<Item = Self> {
        let back = if self.is_empty() {
            0
        } else {
            self.height_usize()
        };
        Rows {
            rect: *self,
            front: 0,
            back,
        }
    }

    /// The single-column rects of `self`, left to right. Empty if `self` is empty.
    ///
    /// This is the iterator form of [`Rect::col_rect`]: `rect.cols().nth(i) == Some(rect.col_rect(i))`
    /// for any in-range `i`. Unlike [`Rect::col_rect`], which clamps out-of-range indices to the
    /// last valid column, `cols()` simply stops after `self.width()` rects instead of clamping.
    ///
    /// An empty rectangle (zero width *or* zero height, per [`Rect::is_empty`]) yields no columns
    /// at all, even when its width is nonzero, mirroring [`Rect::rows`]'s handling of a zero
    /// height.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(0, 0, 2, 3);
    /// let cols: Vec<_> = rect.cols().collect();
    /// assert_eq!(cols, &[Rect::from_ltwh(0, 0, 1, 3), Rect::from_ltwh(1, 0, 1, 3)]);
    ///
    /// // A zero-height rect yields no columns, even though its width is nonzero.
    /// let empty = Rect::from_ltwh(0, 0, 3, 0);
    /// assert_eq!(empty.cols().count(), 0);
    /// ```
    #[must_use]
    pub fn cols(&self) -> impl ExactSizeIterator<Item = Self> {
        let back = if self.is_empty() {
            0
        } else {
            self.width_usize()
        };
        Cols {
            rect: *self,
            front: 0,
            back,
        }
    }

    /// The positions of `self`, grouped per row (top to bottom, left to right within a row).
    ///
    /// Flattening the result yields the same sequence as [`Rect::pos_iter`]; grouping simply
    /// exposes the row boundaries, which is convenient for building output line by line. Each
    /// inner iterator is the [`Rect::pos_iter`] of the corresponding [`Rect::rows`] entry, so an
    /// empty `self` (see [`Rect::rows`]) yields no rows at all.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(0, 0, 2, 2);
    /// let mut lines = Vec::new();
    /// for row in rect.row_pos_iter() {
    ///     let line: Vec<_> = row.map(|pos| format!("({},{})", pos.x, pos.y)).collect();
    ///     lines.push(line.join(" "));
    /// }
    /// assert_eq!(lines, &["(0,0) (1,0)", "(0,1) (1,1)"]);
    /// ```
    #[must_use]
    pub fn row_pos_iter(&self) -> impl ExactSizeIterator<Item = impl Iterator<Item = Pos<T>>> {
        self.rows().map(|row| row.pos_iter())
    }

    /// The positions of `self`, grouped per column (left to right, top to bottom within a
    /// column).
    ///
    /// Flattening the result yields the same sequence as [`ColumnMajor::iter_pos`]. Each inner
    /// iterator is the [`ColumnMajor`] position iteration of the corresponding [`Rect::cols`]
    /// entry, so an empty `self` (see [`Rect::cols`]) yields no columns at all.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::{Rect, layout::{ColumnMajor, Layout}};
    ///
    /// let rect = Rect::from_ltwh(0, 0, 2, 2);
    /// let grouped: Vec<Vec<_>> = rect.col_pos_iter().map(Iterator::collect).collect();
    /// let flattened: Vec<_> = grouped.into_iter().flatten().collect();
    /// assert_eq!(flattened, ColumnMajor::iter_pos(rect).collect::<Vec<_>>());
    /// ```
    #[must_use]
    pub fn col_pos_iter(&self) -> impl ExactSizeIterator<Item = impl Iterator<Item = Pos<T>>> {
        self.cols().map(|col| ColumnMajor::iter_pos(col))
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
    /// For an unsigned `T`, this panics in debug builds (wraps in release) if `dx`/`dy` exceed
    /// this rectangle's `x`/`y`, since the left/top edge would need to move below zero.
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
    /// For an unsigned `T`, this panics in debug builds (wraps in release) if `dx + dx`/`dy + dy`
    /// exceed this rectangle's `w`/`h`.
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

    /// Returns a rectangle shrunk asymmetrically by `top`, `right`, `bottom`, and `left`.
    ///
    /// Unlike [`Rect::shrink`], the amount removed from each edge can differ. The operation
    /// saturates instead of overflowing/panicking: if the insets would push an edge past the
    /// opposite edge (e.g. on a too-small rectangle), the result collapses to a zero-size
    /// rectangle at the point where the edges met, anchored by the top/left insets.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(2, 2, 6, 6);
    /// assert_eq!(rect.inset(1, 2, 1, 2), Rect::from_ltwh(4, 3, 2, 4));
    ///
    /// // Insets larger than the rectangle saturate to an empty rectangle instead of panicking.
    /// let small = Rect::from_ltwh(0u16, 0, 4, 4);
    /// assert_eq!(small.inset(0, 10, 0, 0), Rect::from_ltwh(0, 0, 0, 4));
    /// ```
    #[must_use]
    pub fn inset(&self, top: T, right: T, bottom: T, left: T) -> Self {
        let l = self.x.saturating_add(left);
        let t = self.y.saturating_add(top);
        let r = self.right().saturating_sub(right).max(l);
        let b = self.bottom().saturating_sub(bottom).max(t);

        Self {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
        }
    }

    /// Returns a rectangle grown asymmetrically by `top`, `right`, `bottom`, and `left`.
    ///
    /// Unlike [`Rect::inflate`], the amount added to each edge can differ. The operation
    /// saturates at `T::MIN`/`T::MAX` instead of overflowing/panicking.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let rect = Rect::from_ltwh(4, 3, 2, 4);
    /// assert_eq!(rect.outset(1, 2, 1, 2), Rect::from_ltwh(2, 2, 6, 6));
    /// ```
    #[must_use]
    pub fn outset(&self, top: T, right: T, bottom: T, left: T) -> Self {
        let l = self.x.saturating_sub(left);
        let t = self.y.saturating_sub(top);
        let r = self.right().saturating_add(right).max(l);
        let b = self.bottom().saturating_add(bottom).max(t);

        Self {
            x: l,
            y: t,
            w: r - l,
            h: b - t,
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

    /// Returns `self` moved so it fits inside `bounds`, keeping its size.
    ///
    /// Unlike [`Rect::intersect`], this never shrinks `self`; it only translates it. If `self` is
    /// larger than `bounds` on an axis, it is anchored to `bounds`'s top-left on that axis instead
    /// (it cannot both keep its size and fit). The operation is saturating, so it never panics or
    /// overflows, even for unsigned `T`.
    ///
    /// This is the common "keep this popup/viewport on screen" operation.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let bounds = Rect::from_ltwh(0, 0, 10, 10);
    ///
    /// // Already inside: unchanged.
    /// let inside = Rect::from_ltwh(2, 2, 3, 3);
    /// assert_eq!(inside.clamp_within(bounds), inside);
    ///
    /// // Hanging off the right/bottom edges: slides back in, same size.
    /// let overhanging = Rect::from_ltwh(8, 8, 4, 4);
    /// assert_eq!(overhanging.clamp_within(bounds), Rect::from_ltwh(6, 6, 4, 4));
    ///
    /// // Larger than bounds: anchored to the top-left, saturating instead of shrinking.
    /// let too_big = Rect::from_ltwh(0, 0, 20, 20);
    /// assert_eq!(too_big.clamp_within(bounds), Rect::from_ltwh(0, 0, 20, 20));
    /// ```
    #[must_use]
    pub fn clamp_within(&self, bounds: Self) -> Self {
        let max_x = bounds.right().saturating_sub(self.w).max(bounds.left());
        let max_y = bounds.bottom().saturating_sub(self.h).max(bounds.top());

        Self {
            x: self.x.clamp(bounds.left(), max_x),
            y: self.y.clamp(bounds.top(), max_y),
            w: self.w,
            h: self.h,
        }
    }

    /// Returns `self`'s size centered within `bounds`, then clamped via [`Rect::clamp_within`].
    ///
    /// If `self` is larger than `bounds` on an axis, it is anchored to `bounds`'s top-left on that
    /// axis, matching [`Rect::clamp_within`]'s saturating behavior.
    ///
    /// ## Examples
    ///
    /// ```rust
    /// use ixy::Rect;
    ///
    /// let bounds = Rect::from_ltwh(0, 0, 10, 10);
    /// let popup = Rect::from_ltwh(0, 0, 4, 4);
    /// assert_eq!(popup.centered_in(bounds), Rect::from_ltwh(3, 3, 4, 4));
    /// ```
    #[must_use]
    pub fn centered_in(&self, bounds: Self) -> Self {
        let two = T::ONE + T::ONE;
        let dx = bounds.width().saturating_sub(self.w) / two;
        let dy = bounds.height().saturating_sub(self.h) / two;

        Self {
            x: bounds.left().saturating_add(dx),
            y: bounds.top().saturating_add(dy),
            w: self.w,
            h: self.h,
        }
        .clamp_within(bounds)
    }
}

/// Iterator over the single-row rects of a [`Rect<T>`], top to bottom.
///
/// Returned by [`Rect::rows`].
struct Rows<T: Int> {
    rect: Rect<T>,
    front: usize,
    back: usize,
}

impl<T: Int> Iterator for Rows<T> {
    type Item = Rect<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let row = self.rect.row_rect(self.front);
        self.front += 1;
        Some(row)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for Rows<T> {
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<T: Int> DoubleEndedIterator for Rows<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(self.rect.row_rect(self.back))
    }
}

impl<T: Int> core::iter::FusedIterator for Rows<T> {}

/// Iterator over the single-column rects of a [`Rect<T>`], left to right.
///
/// Returned by [`Rect::cols`].
struct Cols<T: Int> {
    rect: Rect<T>,
    front: usize,
    back: usize,
}

impl<T: Int> Iterator for Cols<T> {
    type Item = Rect<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        let col = self.rect.col_rect(self.front);
        self.front += 1;
        Some(col)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

impl<T: Int> ExactSizeIterator for Cols<T> {
    fn len(&self) -> usize {
        self.back - self.front
    }
}

impl<T: Int> DoubleEndedIterator for Cols<T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.front >= self.back {
            return None;
        }
        self.back -= 1;
        Some(self.rect.col_rect(self.back))
    }
}

impl<T: Int> core::iter::FusedIterator for Cols<T> {}

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

impl<T: Int> ops::Mul<Size<T>> for Rect<T> {
    type Output = Self;

    /// Scales the rectangle's position and dimensions per-axis: `x`/`w` by `rhs.width`, and
    /// `y`/`h` by `rhs.height`.
    ///
    /// Unlike the uniform `Rect * T`, this allows non-uniform scaling, such as converting a
    /// cell-space rectangle to pixel-space when cells aren't square.
    fn mul(self, rhs: Size<T>) -> Self::Output {
        Self {
            x: self.x * rhs.width,
            y: self.y * rhs.height,
            w: self.w * rhs.width,
            h: self.h * rhs.height,
        }
    }
}

impl<T: Int> ops::MulAssign<Size<T>> for Rect<T> {
    fn mul_assign(&mut self, rhs: Size<T>) {
        self.x *= rhs.width;
        self.y *= rhs.height;
        self.w *= rhs.width;
        self.h *= rhs.height;
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
    use alloc::{string::ToString, vec, vec::Vec};

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

    // Regression tests for https://github.com/crates-lurey-io/retroglyph/issues/879: `right()`/
    // `bottom()` used plain `+`, so any Rect whose `x + w` (or `y + h`) exceeds `T::MAX` panicked
    // in debug builds, even though callers like `clamp_within`/`inset`/`outset` document
    // saturating, non-panicking behavior and rely on these two methods internally.

    #[test]
    fn right_saturates_on_unsigned_overflow() {
        let rect = Rect::new(50_000u16, 0, 40_000, 10);
        assert_eq!(rect.right(), u16::MAX);
    }

    #[test]
    fn bottom_saturates_on_unsigned_overflow() {
        let rect = Rect::new(0u16, 50_000, 10, 40_000);
        assert_eq!(rect.bottom(), u16::MAX);
    }

    #[test]
    fn corners_saturate_on_unsigned_overflow() {
        let rect = Rect::new(50_000u16, 50_000, 40_000, 40_000);
        assert_eq!(rect.top_right(), Pos::new(u16::MAX, 50_000));
        assert_eq!(rect.bottom_right(), Pos::new(u16::MAX, u16::MAX));
        assert_eq!(rect.bottom_left(), Pos::new(50_000, u16::MAX));
    }

    #[test]
    fn contains_does_not_panic_when_right_overflows() {
        let rect = Rect::new(50_000u16, 0, 40_000, 10);
        assert!(rect.contains(60_000, 5));
        assert!(!rect.contains(10, 5));
    }

    #[test]
    fn contains_rect_does_not_panic_when_right_overflows() {
        let rect = Rect::new(50_000u16, 0, 40_000, 10);
        assert!(rect.contains_rect(Rect::new(60_000u16, 0, 100, 10)));
    }

    #[test]
    fn intersect_does_not_panic_when_right_overflows() {
        let a = Rect::new(50_000u16, 0, 40_000, 10);
        let b = Rect::new(60_000u16, 0, 100, 10);
        assert_eq!(a.intersect(b), Rect::from_ltwh(60_000, 0, 100, 10));
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
    fn inset_asymmetric() {
        let rect = Rect::from_ltwh(2, 2, 6, 6);
        assert_eq!(rect.inset(1, 2, 1, 2), Rect::from_ltwh(4, 3, 2, 4));
    }

    #[test]
    fn inset_matches_shrink_when_uniform() {
        let rect = Rect::from_ltwh(1, 1, 6, 6);
        assert_eq!(rect.inset(1, 1, 1, 1), rect.shrink(1, 1));
    }

    #[test]
    fn inset_zero_is_identity() {
        let rect = Rect::from_ltwh(1, 2, 3, 4);
        assert_eq!(rect.inset(0, 0, 0, 0), rect);
    }

    #[test]
    fn inset_saturates_on_unsigned_underflow() {
        let rect = Rect::from_ltwh(0u16, 0, 4, 4);
        assert_eq!(rect.inset(0, 10, 0, 0), Rect::from_ltwh(0, 0, 0, 4));
        assert_eq!(rect.inset(0, 0, 10, 0), Rect::from_ltwh(0, 0, 4, 0));
    }

    #[test]
    fn inset_saturates_when_left_and_right_cross() {
        let rect = Rect::from_ltwh(0u16, 0, 4, 4);
        assert_eq!(rect.inset(0, 10, 0, 10), Rect::from_ltwh(10, 0, 0, 4));
    }

    #[test]
    fn inset_does_not_panic_when_self_right_overflows() {
        // Regression test for https://github.com/crates-lurey-io/retroglyph/issues/879.
        let rect = Rect::new(50_000u16, 0, 40_000, 10);
        assert_eq!(
            rect.inset(1, 1, 1, 1),
            Rect::from_ltwh(50_001, 1, 15_533, 8)
        );
    }

    #[test]
    fn outset_asymmetric() {
        let rect = Rect::from_ltwh(4, 3, 2, 4);
        assert_eq!(rect.outset(1, 2, 1, 2), Rect::from_ltwh(2, 2, 6, 6));
    }

    #[test]
    fn outset_matches_inflate_when_uniform() {
        let rect = Rect::from_ltwh(3, 3, 5, 5);
        assert_eq!(rect.outset(1, 1, 1, 1), rect.inflate(1, 1));
    }

    #[test]
    fn outset_saturates_on_unsigned_underflow() {
        // Requesting a left outset of 10 from x=1 can only grow to x=0 (saturating), so the
        // effective growth is capped at 1, not the full requested 10.
        let rect = Rect::from_ltwh(1u16, 1, 4, 4);
        assert_eq!(rect.outset(0, 0, 0, 10), Rect::from_ltwh(0, 1, 5, 4));
    }

    #[test]
    fn outset_saturates_at_max() {
        let rect = Rect::from_ltwh(u8::MAX - 2, 0, 2, 2);
        assert_eq!(rect.outset(0, 10, 0, 0).right(), u8::MAX);
    }

    #[test]
    fn outset_does_not_panic_when_self_right_overflows() {
        // Regression test for https://github.com/crates-lurey-io/retroglyph/issues/879.
        let rect = Rect::new(50_000u16, 0, 40_000, 10);
        assert_eq!(
            rect.outset(1, 1, 1, 1),
            Rect::from_ltwh(49_999, 0, 15_536, 11)
        );
    }

    #[test]
    fn inset_then_outset_is_identity_within_bounds() {
        let rect = Rect::from_ltwh(5, 5, 10, 10);
        assert_eq!(rect.inset(1, 2, 3, 4).outset(1, 2, 3, 4), rect);
    }

    #[test]
    fn clamp_within_already_inside_is_unchanged() {
        let bounds = Rect::from_ltwh(0, 0, 10, 10);
        let inside = Rect::from_ltwh(2, 2, 3, 3);
        assert_eq!(inside.clamp_within(bounds), inside);
    }

    #[test]
    fn clamp_within_slides_back_into_bounds() {
        let bounds = Rect::from_ltwh(0, 0, 10, 10);
        let overhanging = Rect::from_ltwh(8, 8, 4, 4);
        assert_eq!(
            overhanging.clamp_within(bounds),
            Rect::from_ltwh(6, 6, 4, 4)
        );
    }

    #[test]
    fn clamp_within_negative_position_slides_forward() {
        let bounds = Rect::from_ltwh(0, 0, 10, 10);
        let off_screen = Rect::from_ltwh(-5, -5, 4, 4);
        assert_eq!(off_screen.clamp_within(bounds), Rect::from_ltwh(0, 0, 4, 4));
    }

    #[test]
    fn clamp_within_larger_than_bounds_anchors_top_left() {
        let bounds = Rect::from_ltwh(0, 0, 10, 10);
        let too_big = Rect::from_ltwh(0, 0, 20, 20);
        assert_eq!(too_big.clamp_within(bounds), Rect::from_ltwh(0, 0, 20, 20));
    }

    #[test]
    fn clamp_within_unsigned_does_not_panic() {
        let bounds = Rect::from_ltwh(5u16, 5, 10, 10);
        let too_big = Rect::from_ltwh(0u16, 0, 30, 30);
        assert_eq!(too_big.clamp_within(bounds), Rect::from_ltwh(5, 5, 30, 30));
    }

    #[test]
    fn clamp_within_offset_bounds() {
        let bounds = Rect::from_ltwh(100, 100, 10, 10);
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(rect.clamp_within(bounds), Rect::from_ltwh(100, 100, 4, 4));
    }

    #[test]
    fn clamp_within_does_not_panic_when_bounds_right_overflows() {
        // Regression test for https://github.com/crates-lurey-io/retroglyph/issues/879: bounds
        // whose own `x + w` exceeds `u16::MAX` used to panic instead of saturating.
        let bounds = Rect::new(50_000u16, 0, 40_000, 10);
        assert_eq!(bounds.clamp_within(bounds), bounds);
    }

    #[test]
    fn clamp_within_clamps_correctly_when_bounds_right_overflows() {
        let bounds = Rect::new(50_000u16, 0, 40_000, 10);
        let hanging = Rect::new(63_000u16, 8, 4_000, 5);
        assert_eq!(
            hanging.clamp_within(bounds),
            Rect::from_ltwh(61_535, 5, 4_000, 5)
        );
    }

    #[test]
    fn centered_in_even_bounds() {
        let bounds = Rect::from_ltwh(0, 0, 10, 10);
        let popup = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(popup.centered_in(bounds), Rect::from_ltwh(3, 3, 4, 4));
    }

    #[test]
    fn centered_in_odd_bounds_rounds_towards_top_left() {
        let bounds = Rect::from_ltwh(0, 0, 9, 9);
        let popup = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(popup.centered_in(bounds), Rect::from_ltwh(2, 2, 4, 4));
    }

    #[test]
    fn centered_in_offset_bounds() {
        let bounds = Rect::from_ltwh(5, 5, 10, 10);
        let popup = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(popup.centered_in(bounds), Rect::from_ltwh(8, 8, 4, 4));
    }

    #[test]
    fn centered_in_larger_than_bounds_anchors_top_left() {
        let bounds = Rect::from_ltwh(0, 0, 4, 4);
        let too_big = Rect::from_ltwh(0, 0, 10, 10);
        assert_eq!(too_big.centered_in(bounds), Rect::from_ltwh(0, 0, 10, 10));
    }

    #[test]
    fn centered_in_unsigned_does_not_panic() {
        let bounds = Rect::from_ltwh(0u16, 0, 4, 4);
        let too_big = Rect::from_ltwh(0u16, 0, 10, 10);
        assert_eq!(too_big.centered_in(bounds), Rect::from_ltwh(0, 0, 10, 10));
    }

    #[test]
    fn centered_in_does_not_panic_when_bounds_right_overflows() {
        // Regression test for https://github.com/crates-lurey-io/retroglyph/issues/879.
        let bounds = Rect::new(50_000u16, 0, 40_000, 10);
        let popup = Rect::new(50_000u16, 0, 100, 5);
        assert_eq!(
            popup.centered_in(bounds),
            Rect::from_ltwh(65_435, 2, 100, 5)
        );
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
    fn rows_non_origin_rect() {
        let rect = Rect::from_ltwh(2, 3, 4, 3);
        let rows: Vec<_> = rect.rows().collect();
        assert_eq!(
            rows,
            &[
                Rect::from_ltwh(2, 3, 4, 1),
                Rect::from_ltwh(2, 4, 4, 1),
                Rect::from_ltwh(2, 5, 4, 1),
            ]
        );
    }

    #[test]
    fn cols_non_origin_rect() {
        let rect = Rect::from_ltwh(2, 3, 3, 4);
        let cols: Vec<_> = rect.cols().collect();
        assert_eq!(
            cols,
            &[
                Rect::from_ltwh(2, 3, 1, 4),
                Rect::from_ltwh(3, 3, 1, 4),
                Rect::from_ltwh(4, 3, 1, 4),
            ]
        );
    }

    #[test]
    fn rows_len_before_and_after_partial_consumption() {
        let rect = Rect::from_ltwh(0, 0, 4, 5);
        let mut rows = rect.rows();
        assert_eq!(rows.len(), 5);
        rows.next();
        rows.next();
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.count(), 3);
    }

    #[test]
    fn cols_len_before_and_after_partial_consumption() {
        let rect = Rect::from_ltwh(0, 0, 5, 4);
        let mut cols = rect.cols();
        assert_eq!(cols.len(), 5);
        cols.next();
        cols.next();
        assert_eq!(cols.len(), 3);
        assert_eq!(cols.count(), 3);
    }

    #[test]
    fn rows_len_after_next_back() {
        // `Rows` is a private iterator struct backing `Rect::rows`; constructed directly here
        // (within the same module) to exercise `DoubleEndedIterator`, which is implemented on the
        // concrete type even though the public `Rect::rows` signature only promises
        // `ExactSizeIterator`.
        let rect = Rect::from_ltwh(0, 0, 2, 4);
        let mut rows = Rows {
            rect,
            front: 0,
            back: rect.height_usize(),
        };
        assert_eq!(rows.len(), 4);
        assert_eq!(rows.next_back(), Some(Rect::from_ltwh(0, 3, 2, 1)));
        assert_eq!(rows.len(), 3);
        assert_eq!(rows.next(), Some(Rect::from_ltwh(0, 0, 2, 1)));
        assert_eq!(rows.len(), 2);
    }

    #[test]
    fn cols_len_after_next_back() {
        let rect = Rect::from_ltwh(0, 0, 4, 2);
        let mut cols = Cols {
            rect,
            front: 0,
            back: rect.width_usize(),
        };
        assert_eq!(cols.len(), 4);
        assert_eq!(cols.next_back(), Some(Rect::from_ltwh(3, 0, 1, 2)));
        assert_eq!(cols.len(), 3);
        assert_eq!(cols.next(), Some(Rect::from_ltwh(0, 0, 1, 2)));
        assert_eq!(cols.len(), 2);
    }

    #[test]
    fn rows_empty_zero_width() {
        let rect = Rect::from_ltwh(0, 0, 0, 3);
        assert!(rect.is_empty());
        assert_eq!(rect.rows().count(), 0);
        assert_eq!(rect.rows().len(), 0);
    }

    #[test]
    fn rows_empty_zero_height() {
        let rect = Rect::from_ltwh(0, 0, 3, 0);
        assert_eq!(rect.rows().count(), 0);
    }

    #[test]
    fn rows_empty_zero_by_zero() {
        assert_eq!(Rect::<i32>::EMPTY.rows().count(), 0);
        assert_eq!(Rect::<i32>::EMPTY.cols().count(), 0);
    }

    #[test]
    fn cols_empty_zero_width() {
        let rect = Rect::from_ltwh(0, 0, 0, 3);
        assert_eq!(rect.cols().count(), 0);
    }

    #[test]
    fn cols_empty_zero_height() {
        let rect = Rect::from_ltwh(0, 0, 3, 0);
        assert!(rect.is_empty());
        assert_eq!(rect.cols().count(), 0);
        assert_eq!(rect.cols().len(), 0);
    }

    #[test]
    fn rows_1x1_rect() {
        let rect = Rect::from_ltwh(5, 5, 1, 1);
        let rows: Vec<_> = rect.rows().collect();
        assert_eq!(rows, &[rect]);
    }

    #[test]
    fn cols_1x1_rect() {
        let rect = Rect::from_ltwh(5, 5, 1, 1);
        let cols: Vec<_> = rect.cols().collect();
        assert_eq!(cols, &[rect]);
    }

    #[test]
    fn rows_agrees_with_row_rect_in_range() {
        let rect = Rect::from_ltwh(1, 2, 4, 3);
        for (i, row) in rect.rows().enumerate() {
            assert_eq!(Some(row), Some(rect.row_rect(i)));
        }
    }

    #[test]
    fn cols_agrees_with_col_rect_in_range() {
        let rect = Rect::from_ltwh(1, 2, 4, 3);
        for (i, col) in rect.cols().enumerate() {
            assert_eq!(Some(col), Some(rect.col_rect(i)));
        }
    }

    #[test]
    fn row_pos_iter_flattens_to_pos_iter() {
        let rect = Rect::from_ltwh(1, 2, 3, 2);
        let flattened: Vec<_> = rect.row_pos_iter().flatten().collect();
        let via_pos_iter: Vec<_> = rect.pos_iter().collect();
        assert_eq!(flattened, via_pos_iter);
    }

    #[test]
    fn row_pos_iter_len_and_grouping() {
        let rect = Rect::from_ltwh(0, 0, 2, 3);
        let iter = rect.row_pos_iter();
        assert_eq!(iter.len(), 3);
        let grouped: Vec<Vec<_>> = iter.map(Iterator::collect).collect();
        assert_eq!(
            grouped,
            vec![
                vec![Pos::new(0, 0), Pos::new(1, 0)],
                vec![Pos::new(0, 1), Pos::new(1, 1)],
                vec![Pos::new(0, 2), Pos::new(1, 2)],
            ]
        );
    }

    #[test]
    fn row_pos_iter_empty() {
        let rect = Rect::from_ltwh(0, 0, 0, 3);
        assert_eq!(rect.row_pos_iter().count(), 0);
    }

    #[test]
    fn col_pos_iter_flattens_to_column_major_iter_pos() {
        use crate::layout::{ColumnMajor, Layout};

        let rect = Rect::from_ltwh(1, 2, 3, 2);
        let flattened: Vec<_> = rect.col_pos_iter().flatten().collect();
        let via_column_major: Vec<_> = ColumnMajor::iter_pos(rect).collect();
        assert_eq!(flattened, via_column_major);
    }

    #[test]
    fn col_pos_iter_len_and_grouping() {
        let rect = Rect::from_ltwh(0, 0, 3, 2);
        let iter = rect.col_pos_iter();
        assert_eq!(iter.len(), 3);
        let grouped: Vec<Vec<_>> = iter.map(Iterator::collect).collect();
        assert_eq!(
            grouped,
            vec![
                vec![Pos::new(0, 0), Pos::new(0, 1)],
                vec![Pos::new(1, 0), Pos::new(1, 1)],
                vec![Pos::new(2, 0), Pos::new(2, 1)],
            ]
        );
    }

    #[test]
    fn col_pos_iter_empty() {
        let rect = Rect::from_ltwh(0, 0, 3, 0);
        assert_eq!(rect.col_pos_iter().count(), 0);
    }

    #[test]
    fn rows_u16_coordinates() {
        let rect = Rect::from_ltwh(50_000u16, 0, 3, 2);
        let rows: Vec<_> = rect.rows().collect();
        assert_eq!(
            rows,
            &[
                Rect::from_ltwh(50_000u16, 0, 3, 1),
                Rect::from_ltwh(50_000u16, 1, 3, 1),
            ]
        );
    }

    #[test]
    fn cols_u16_coordinates() {
        let rect = Rect::from_ltwh(50_000u16, 0, 3, 2);
        let cols: Vec<_> = rect.cols().collect();
        assert_eq!(
            cols,
            &[
                Rect::from_ltwh(50_000u16, 0, 1, 2),
                Rect::from_ltwh(50_001u16, 0, 1, 2),
                Rect::from_ltwh(50_002u16, 0, 1, 2),
            ]
        );
    }

    #[test]
    fn rows_double_ended_fused() {
        let rect = Rect::from_ltwh(0, 0, 1, 3);
        let mut rows = Rows {
            rect,
            front: 0,
            back: rect.height_usize(),
        };
        rows.next();
        rows.next();
        rows.next();
        assert_eq!(rows.next(), None);
        assert_eq!(rows.next(), None);
        assert_eq!(rows.next_back(), None);
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
    fn mul_size_non_uniform() {
        use crate::Size;

        let rect = Rect::from_ltwh(1, 2, 3, 4);
        let new_rect = rect * Size::new(2, 3);
        assert_eq!(new_rect, Rect::from_ltwh(2, 6, 6, 12));
    }

    #[test]
    fn mul_assign_size_non_uniform() {
        use crate::Size;

        let mut rect = Rect::from_ltwh(1, 2, 3, 4);
        rect *= Size::new(2, 3);
        assert_eq!(rect, Rect::from_ltwh(2, 6, 6, 12));
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

    #[test]
    fn row_rect_out_of_bounds_clamps_to_last_row() {
        // Regression test for https://github.com/crates-lurey-io/ixy/issues/9: an out-of-bounds
        // row used to escape the rectangle entirely instead of clamping, contradicting the doc.
        let rect = Rect::new(0u16, 0, 4, 4);
        assert_eq!(rect.row_rect(10), rect.row_rect(3));
        assert_eq!(rect.row_rect(10), Rect::from_ltwh(0, 3, 4, 1));
    }

    #[test]
    fn col_rect_out_of_bounds_clamps_to_last_col() {
        let rect = Rect::new(0u16, 0, 4, 4);
        assert_eq!(rect.col_rect(10), rect.col_rect(3));
        assert_eq!(rect.col_rect(10), Rect::from_ltwh(3, 0, 1, 4));
    }

    #[test]
    fn row_rect_result_always_within_bounds() {
        let rect = Rect::from_ltwh(0u16, 0, 4, 4);
        for row in 0..10 {
            let sub = rect.row_rect(row);
            assert!(
                rect.contains_rect(sub),
                "row_rect({row}) = {sub:?} escaped {rect:?}"
            );
        }
    }

    #[test]
    fn col_rect_result_always_within_bounds() {
        let rect = Rect::from_ltwh(0u16, 0, 4, 4);
        for col in 0..10 {
            let sub = rect.col_rect(col);
            assert!(
                rect.contains_rect(sub),
                "col_rect({col}) = {sub:?} escaped {rect:?}"
            );
        }
    }

    #[test]
    fn row_rect_last_valid_row_touches_bottom_edge() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(rect.row_rect(3).bottom(), rect.bottom());
    }

    #[test]
    fn col_rect_last_valid_col_touches_right_edge() {
        let rect = Rect::from_ltwh(0, 0, 4, 4);
        assert_eq!(rect.col_rect(3).right(), rect.right());
    }
}
