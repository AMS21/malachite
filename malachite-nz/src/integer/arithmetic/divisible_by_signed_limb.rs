use integer::Integer;
use malachite_base::num::{DivisibleBy, UnsignedAbs};
use platform::SignedLimb;

impl<'a> DivisibleBy<SignedLimb> for &'a Integer {
    /// Returns whether an `Integer` is divisible by a `SignedLimb`; in other words, whether the
    /// `Integer` is a multiple of the `SignedLimb`. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero.
    ///
    /// This method is more efficient than finding a remainder and checking whether it's zero.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.divisible_by(0i32), true);
    ///     assert_eq!(Integer::from(100).divisible_by(-3i32), false);
    ///     assert_eq!(Integer::from(-102).divisible_by(3i32), true);
    /// }
    /// ```
    fn divisible_by(self, other: SignedLimb) -> bool {
        self.abs.divisible_by(other.unsigned_abs())
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivisibleBy<i32> for &'a Integer {
    #[inline]
    fn divisible_by(self, other: i32) -> bool {
        self.divisible_by(SignedLimb::from(other))
    }
}

impl<'a> DivisibleBy<&'a Integer> for SignedLimb {
    /// Returns whether a `SignedLimb` is divisible by an `Integer`; in other words, whether the
    /// `SignedLimb` is a multiple of the `Integer`. This means that zero is divisible by any
    /// number, including zero; but a nonzero number is never divisible by zero.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{DivisibleBy, Zero};
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(0i32.divisible_by(&Integer::ZERO), true);
    ///     assert_eq!((-100i32).divisible_by(&Integer::from(3)), false);
    ///     assert_eq!(102i32.divisible_by(&Integer::from(-3)), true);
    /// }
    /// ```
    fn divisible_by(self, other: &'a Integer) -> bool {
        self.unsigned_abs().divisible_by(&other.abs)
    }
}

#[cfg(feature = "64_bit_limbs")]
impl<'a> DivisibleBy<&'a Integer> for i32 {
    #[inline]
    fn divisible_by(self, other: &'a Integer) -> bool {
        SignedLimb::from(self).divisible_by(other)
    }
}
