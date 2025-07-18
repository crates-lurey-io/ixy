//! Sealed traits implemented by all of Rust's primitive integer types.
//!
//! These traits provides a common interface for working with integers generically.

use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Not, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use crate::internal::Sealed;

/// Generic trait for the built-in Rust integer types (e.g. `u8`, `u32`, `i64`, `isize`, ...).
///
/// Unlike traits provided by crates like `num_traits`, it is _sealed_.
///
/// This trait exists to avoid/allow:
/// 1. Having a mandatory dependency on a crate (such as `num_traits`);
/// 1. Worrying about the trait being implemented for types that are not integers;
/// 1. Adding new methods that might only be useful in the context of 2D integer geometry.
#[allow(private_bounds)]
pub trait Int:
    Sealed
    + Sized
    + Copy
    + PartialEq
    + Eq
    + PartialOrd
    + Ord
    + Not<Output = Self>
    + BitAnd<Output = Self>
    + BitAndAssign
    + BitOr<Output = Self>
    + BitOrAssign
    + BitXor<Output = Self>
    + BitXorAssign
    + Shl<Output = Self>
    + ShlAssign
    + Shr<Output = Self>
    + ShrAssign
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div<Output = Self>
    + DivAssign
    + Rem<Output = Self>
    + RemAssign
{
    /// The integer value of `0`.
    const ZERO: Self;

    /// The integer value of `1`.
    const ONE: Self;

    /// The minimum value of the integer type.
    const MIN: Self;

    /// The maximum value of the integer type.
    const MAX: Self;

    /// Converts the value of `self` to a [`usize`].
    ///
    /// In debug mode, this will panic if the value cannot be represented by a [`usize`], and in
    /// release mode, the result is clamped.
    fn to_usize(&self) -> usize {
        const MSG: &str = "Value out of bounds for usize";
        #[cfg(not(coverage))]
        {
            self.checked_to_usize().unwrap_or_else(|| {
                let val = *self;
                debug_assert!(val >= Self::ZERO && val <= Self::MAX as Self, "{MSG}");
                self.saturating_to_usize()
            })
        }
        #[cfg(coverage)]
        {
            self.checked_to_usize().expect(MSG)
        }
    }

    /// Converts the value of `self` to a [`usize`].
    ///
    /// If the value cannot be represented by a [`usize`], then it is clamped.
    fn saturating_to_usize(&self) -> usize {
        self.checked_to_usize().unwrap_or_else(|| {
            // This is a fallback for when the value is negative or too large.
            if *self < Self::ZERO { 0 } else { usize::MAX }
        })
    }

    /// Converts the value of `self` to a [`usize`].
    ///
    /// If the value cannot be represented by a [`usize`], then [`None`] is returned.
    fn checked_to_usize(&self) -> Option<usize>;

    /// Converts a [`usize`] to the integer type `Self`.
    ///
    /// In debug mode, this will panic if the value cannot be represented by `Self`, and in
    /// release mode, the result is clamped.
    #[must_use]
    fn from_usize(value: usize) -> Self {
        const MSG: &str = "Value out of bounds for integer type";
        #[cfg(not(coverage))]
        {
            Self::checked_from_usize(value).unwrap_or_else(|| {
                debug_assert!(value <= Self::MAX.to_usize(), "{MSG}",);
                Self::saturating_from_usize(value)
            })
        }
        #[cfg(coverage)]
        {
            Self::checked_from_usize(value).expect(MSG)
        }
    }

    /// Converts a [`usize`] to the integer type `Self`.
    ///
    /// If the value cannot be represented by `Self`, then it is clamped to the maximum value of
    /// `Self`.
    #[must_use]
    fn saturating_from_usize(value: usize) -> Self {
        Self::checked_from_usize(value).unwrap_or(Self::MAX)
    }

    /// Converts a [`usize`] to the integer type `Self`.
    ///
    /// If the value cannot be represented by `Self`, then [`None`] is returned.
    fn checked_from_usize(value: usize) -> Option<Self>;
}

/// Generic trait for the built-in Rust signed integer types (e.g. `i8`, `i32`, `isize`, ...).
///
/// Unlike traits provided by crates like `num_traits`, it is _sealed_.
///
/// See [`Int`] for design decisions around this trait.
pub trait SignedInt: Int + Neg<Output = Self> {
    /// The negative value of `1`.
    const NEG_ONE: Self;
}

macro_rules! impl_int {
  ($($t:ty),*) => {
    $(
      impl Sealed for $t {}

      impl Int for $t {
        const ZERO: Self = 0;
        const ONE: Self = 1;
        const MIN: Self = <$t>::MIN;
        const MAX: Self = <$t>::MAX;

        fn checked_to_usize(&self) -> Option<usize> {
          usize::try_from(*self).ok()
        }

        fn checked_from_usize(value: usize) -> Option<Self> {
          Self::try_from(value).ok()
        }
      }
    )*
  };
}

macro_rules! impl_signed_int {
  ($($t:ty),*) => {
    $(
      impl SignedInt for $t {
        const NEG_ONE: Self = -1;
      }
    )*
  };
}

#[rustfmt::skip]
impl_int!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

#[rustfmt::skip]
impl_signed_int!(
    i8,
    i16,
    i32,
    i64,
    i128,
    isize
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u8_to_usize() {
        let value = 255u8;
        assert_eq!(value.to_usize(), 255);
        assert_eq!(value.checked_to_usize(), Some(255));
        assert_eq!(value.saturating_to_usize(), 255);
    }

    #[test]
    fn u8_from_usize_ok() {
        let value = 255usize;
        assert_eq!(u8::from_usize(value), 255);
        assert_eq!(u8::checked_from_usize(value), Some(255));
        assert_eq!(u8::saturating_from_usize(value), 255);
    }

    #[test]
    fn u8_from_usize_out_of_bounds() {
        let value = 256usize;
        assert_eq!(u8::checked_from_usize(value), None);
        assert_eq!(u8::saturating_from_usize(value), 255);
    }

    #[test]
    #[should_panic(expected = "Value out of bounds for integer type")]
    fn u8_from_usize_out_of_bounds_panic() {
        let _ = u8::from_usize(256usize); // This should panic in debug mode
    }

    #[test]
    fn i8_to_usize_ok() {
        let value = 127i8;
        assert_eq!(value.checked_to_usize(), Some(127));
        assert_eq!(value.saturating_to_usize(), 127);
        assert_eq!(value.to_usize(), 127);
    }

    #[test]
    fn i8_to_usize_negative_err() {
        let value = -1i8;
        assert_eq!(value.checked_to_usize(), None);
        assert_eq!(value.saturating_to_usize(), 0);
    }

    #[test]
    #[should_panic(expected = "Value out of bounds for usize")]
    fn i8_to_usize_negative_panic() {
        let _ = (-1i8).to_usize(); // This should panic in debug mode
    }

    #[test]
    fn i8_from_usize_ok() {
        let value = 127usize;
        assert_eq!(i8::from_usize(value), 127);
        assert_eq!(i8::checked_from_usize(value), Some(127));
        assert_eq!(i8::saturating_from_usize(value), 127);
    }

    #[test]
    fn i8_from_usize_out_of_bounds() {
        let value = 128usize;
        assert_eq!(i8::checked_from_usize(value), None);
        assert_eq!(i8::saturating_from_usize(value), 127);
    }

    #[test]
    #[should_panic(expected = "Value out of bounds for integer type")]
    fn i8_from_usize_out_of_bounds_panic() {
        let _ = i8::from_usize(128usize); // This should panic in debug mode
    }

    #[test]
    fn i128_to_usize_saturating_out_of_bounds() {
        let value = i128::MAX;
        assert_eq!(value.checked_to_usize(), None);
        assert_eq!(value.saturating_to_usize(), usize::MAX);
    }
}
