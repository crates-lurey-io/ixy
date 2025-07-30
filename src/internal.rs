use crate::int::Int;

/// Used to seal traits for the crate.
#[doc(hidden)]
pub(super) trait Sealed {}

/// Calculates the greatest common divisor (GCD) of two integers.
///
/// The result is always positive.
pub(super) fn gcd<T: Int>(a: T, b: T) -> T {
    // Use Stein's algorithm for GCD.
    if a == T::ZERO || b == T::ZERO {
        return (a | b).abs();
    }

    let shift = (a | b).trailing_zeros();

    if a == T::MIN && b == T::MIN {
        let result = (1 << shift).abs();
        return T::from_usize(result.to_usize());
    }

    let mut a = a.abs();
    let mut b = b.abs();

    a >>= a.trailing_zeros();
    b >>= b.trailing_zeros();

    while a != b {
        if a > b {
            a -= b;
            a >>= a.trailing_zeros();
        } else {
            b -= a;
            b >>= b.trailing_zeros();
        }
    }

    a << shift
}

/// Returns an approximation of the integer square root of an integer.
pub(super) fn isqrt<T: Int>(n: T) -> T {
    if n <= T::ZERO {
        return T::ZERO;
    }

    let i2 = T::from_usize(2);
    let mut x = n;
    let mut y = (n + T::ONE) / i2;

    while y < x {
        x = y;
        y = (n / x + x) / i2;
    }

    x
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gcd_basic_cases() {
        assert_eq!(gcd(10, 2), 2);
        assert_eq!(gcd(10, 3), 1);
        assert_eq!(gcd(0, 3), 3);
        assert_eq!(gcd(3, 3), 3);
        assert_eq!(gcd(56, 42), 14);
        assert_eq!(gcd(3, -3), 3);
        assert_eq!(gcd(-6, 3), 3);
        assert_eq!(gcd(-4, -2), 2);
    }

    #[test]
    fn test_gcd_zero_cases() {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(0, -7), 7);
        assert_eq!(gcd(-7, 0), 7);
    }

    #[test]
    fn test_gcd_negative_numbers() {
        assert_eq!(gcd(-10, -2), 2);
        assert_eq!(gcd(-10, 3), 1);
        assert_eq!(gcd(10, -3), 1);
        assert_eq!(gcd(-56, 42), 14);
        assert_eq!(gcd(56, -42), 14);
    }

    #[test]
    fn test_gcd_one_and_self() {
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(1, 99), 1);
        assert_eq!(gcd(99, 1), 1);
        assert_eq!(gcd(-1, 99), 1);
        assert_eq!(gcd(99, -1), 1);
    }

    #[test]
    fn test_gcd_large_numbers() {
        assert_eq!(gcd(123_456, 7890), 6);
        assert_eq!(gcd(1_000_000, 2_500_000), 500_000);
    }

    #[test]
    fn test_isqrt() {
        assert_eq!(isqrt(0), 0);
        assert_eq!(isqrt(1), 1);
        assert_eq!(isqrt(4), 2);
        assert_eq!(isqrt(9), 3);
        assert_eq!(isqrt(15), 3);
        assert_eq!(isqrt(16), 4);
        assert_eq!(isqrt(25), 5);
        assert_eq!(isqrt(26), 5);
        assert_eq!(isqrt(100), 10);
        assert_eq!(isqrt(101), 10);
        assert_eq!(isqrt(1_000_000), 1000);
    }
}
