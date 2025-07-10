use crate::int::Int;

/// Represents a size in 2D space, with `width` and `height`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Size<T: Int = i32> {
    /// Width.
    pub width: T,

    /// Height.
    pub height: T,
}

/// A type that has a [`Size`].
pub trait HasSize {
    type Dim: Int;

    /// Returns the size of the object.
    fn size(&self) -> Size<Self::Dim>;

    /// Returns the width of the object.
    fn width(&self) -> Self::Dim {
        self.size().width
    }

    /// Returns the height of the object.
    fn height(&self) -> Self::Dim {
        self.size().height
    }
}

/// A type that can be converted to a [`Size`].
pub trait IntoSize {
    type Dim: Int;

    /// Converts the object into a [`Size`].
    fn into_size(self) -> Size<Self::Dim>;
}

impl<T: Int> HasSize for Size<T> {
    type Dim = T;

    fn size(&self) -> Size<T> {
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
}
