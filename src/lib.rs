//! # Floating Point Comparison
//!
//! This crate provides a custom floating-point number representation (`Float`) that
//! separates a floating-point number into its core components: mantissa, exponent, and sign.
//! The primary purpose is to enable precise floating-point comparisons and benchmarking
//! against other floating-point comparison methods.
//!
//! ## Features
//! - Custom floating-point representation using mantissa, exponent, and sign
//! - Conversion to and from f64
//! - Precise comparison operations
//! - Benchmarking infrastructure for comparison with other methods (e.g., Decimal)
//!
//! ## Example
//! ```
//! use fast_float_compare::Float;
//!
//! let a = Float::from_f64(1.23);
//! let b = Float::from_f64(4.56);
//!
//! assert!(a < b);
//!
//! // Roundtrip conversion
//! let value = 1.23;
//! let raw = Float::from_f64(value).unwrap();
//! let converted = raw.to_f64();
//! assert!((value - converted).abs() < 1e-10);
//! ```

/// A custom floating-point number representation that separates the number into
/// its fundamental components for more precise comparison operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Float {
    /// The significant digits of the number, normalized to remove trailing zeros
    mantissa: u64,
    /// The power of 10 that the mantissa should be multiplied by
    exponent: i16,
    /// The sign of the number (true for positive, false for negative)
    sign: bool,
}

impl Float {
    fn new(mantissa: u64, exponent: i16, sign: bool) -> Self {
        Self {
            mantissa,
            exponent,
            sign,
        }
    }

    /// Creates a new `Float` from a `f64` value.
    ///
    /// This method decomposes the floating-point number into its constituent parts,
    /// normalizing the mantissa to remove trailing zeros and adjusting the exponent
    /// accordingly.
    ///
    /// # Arguments
    /// * `value` - The floating-point `f64` number to decompose.
    ///
    /// # Returns
    /// A `Float` representation of the input number.
    ///
    /// # Example
    /// ```
    /// use fast_float_compare::Float;
    ///
    /// let num = Float::from_f64(1.23);
    /// assert!(num.is_some());
    /// ```
    ///
    /// # Errors
    /// Returns `None` if the number is NaN or infinite.
    pub fn from_f64(value: f64) -> Option<Self> {
        if value.is_nan() || value.is_infinite() {
            return None;
        }

        // Get the bits of the f64
        let bits: u64 = value.to_bits();

        // Get the sign bit
        let sign = bits >> 63 == 0;

        // Get the exponent
        let exponent: i16 = ((bits >> 52) & 0x7ff) as i16;

        // Get the mantissa
        let mantissa = if exponent == 0 {
            // Subnormal number
            (bits & 0xfffffffffffff) << 1
        } else {
            // Normal number
            (bits & 0xfffffffffffff) | 0x10000000000000
        };

        // Return the Float
        Some(Self::new(mantissa, exponent, sign))
    }

    /// Converts a `Float` back to an `f64`.
    ///
    /// This method reconstructs the original floating-point number from its
    /// mantissa, exponent, and sign components.
    ///
    /// # Example
    /// ```
    /// use fast_float_compare::Float;
    ///
    /// let num = Float::from_f64(1.23).unwrap();
    /// let value = num.to_f64();
    /// assert_eq!(value, 1.23);
    /// ```
    ///
    /// # Returns
    /// The reconstructed `f64` value.
    pub fn to_f64(&self) -> f64 {
        let exponent = self.exponent;

        // Calculate the bits
        let sign_bit = if !self.sign { 1u64 << 63 } else { 0 };

        // Handle subnormal numbers
        let (mantissa_bits, exp_bits) = if exponent <= 0 {
            // Subnormal number
            ((self.mantissa >> 1) & 0xfffffffffffff, 0u64)
        } else {
            // Normal number
            (self.mantissa & 0xfffffffffffff, (exponent as u64) << 52)
        };

        let bits = sign_bit | exp_bits | mantissa_bits;

        // Convert bits back to f64
        f64::from_bits(bits)
    }
}

/// Implements the `Ord` trait for `Float`.
/// This trait allows for comparison between `Float` values.
///
/// # Example
/// ```
/// use fast_float_compare::Float;
///
/// let a = Float::from_f64(1.23).unwrap();
/// let b = Float::from_f64(4.56).unwrap();
/// assert!(a < b);
/// ```
impl Ord for Float {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // First compare signs
        match self.sign.cmp(&other.sign) {
            std::cmp::Ordering::Equal => {
                // For same signs, compare based on exponent and mantissa
                if self.sign {
                    // Positive numbers: larger exponent = larger number
                    match self.exponent.cmp(&other.exponent) {
                        std::cmp::Ordering::Equal => self.mantissa.cmp(&other.mantissa),
                        ord => ord,
                    }
                } else {
                    // Negative numbers: larger exponent = smaller number
                    match self.exponent.cmp(&other.exponent) {
                        std::cmp::Ordering::Equal => other.mantissa.cmp(&self.mantissa),
                        ord => ord.reverse(),
                    }
                }
            }

            ord => ord,
        }
    }
}

impl PartialOrd for Float {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nans_and_infs() {
        let nan = Float::from_f64(f64::NAN);
        let inf = Float::from_f64(f64::INFINITY);
        let neg_inf = Float::from_f64(f64::NEG_INFINITY);

        assert!(nan.is_none());
        assert!(inf.is_none());
        assert!(neg_inf.is_none());
    }

    #[test]
    fn test_f64_roundtrip() {
        let test_values = vec![1.23, -4.56, 0.0, 1234.5678, -0.00123, 100.0, -100.0];

        for &value in &test_values {
            let raw = Float::from_f64(value);
            let roundtrip = raw.unwrap().to_f64();
            // Using approximate equality due to potential floating-point precision differences
            assert!(
                value == roundtrip,
                "Failed roundtrip for {}: got {}",
                value,
                roundtrip
            );
        }
    }

    #[test]
    fn test_cmp() {
        let a = Float::from_f64(1.23);
        let b = Float::from_f64(4.56);
        assert!(a < b);

        let c = Float::from_f64(-2.0);
        let d = Float::from_f64(-2.0); // Same value for equality test
        let e = Float::from_f64(1.23); // Same as a for equality test
        let f = Float::from_f64(-5.67);

        // Test less than
        assert!(a < b);
        assert!(c < b);
        assert!(f < c);

        // Test greater than
        assert!(b > a);
        assert!(b > c);
        assert!(c > f);

        // Test less than or equal to
        assert!(a <= b);
        assert!(c <= b);
        assert!(c <= d);
        assert!(a <= e);

        // Test greater than or equal to
        assert!(b >= a);
        assert!(b >= c);
        assert!(d >= c);
        assert!(e >= a);

        // Test equality
        assert!(c == d);
        assert!(a == e);
        assert!(a != b);
        assert!(c != f);
    }
}
