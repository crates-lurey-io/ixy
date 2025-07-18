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
}
