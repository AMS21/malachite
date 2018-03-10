use malachite_base::limbs::limbs_test_zero;
use malachite_base::num::IsPowerOfTwo;
use natural::Natural::{self, Large, Small};

/// Interpreting a slice of `u32`s as the limbs of a `Natural` in ascending order, determines
/// whether that `Natural` is an integer power of 2.
///
/// This function assumes that `limbs` is nonempty and the last (most significant) limb is nonzero.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `limbs.len()`
///
/// # Panics
/// Panics if `limbs` is empty.
///
/// # Example
/// ```
/// use malachite_nz::natural::arithmetic::is_power_of_two::limbs_is_power_of_two;
///
/// assert_eq!(limbs_is_power_of_two(&[3]), false);
/// assert_eq!(limbs_is_power_of_two(&[0, 0b1000]), true);
/// assert_eq!(limbs_is_power_of_two(&[1, 0b1000]), false);
/// assert_eq!(limbs_is_power_of_two(&[0, 0b1010]), false);
/// ```
pub fn limbs_is_power_of_two(limbs: &[u32]) -> bool {
    limbs_test_zero(&limbs[0..limbs.len() - 1]) && limbs.last().unwrap().is_power_of_two()
}

impl IsPowerOfTwo for Natural {
    /// Determines whether a `Natural` is an integer power of 2.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Example
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::num::{IsPowerOfTwo, Zero};
    /// use malachite_nz::natural::Natural;
    /// use std::str::FromStr;
    ///
    /// fn main() {
    ///     assert_eq!(Natural::ZERO.is_power_of_two(), false);
    ///     assert_eq!(Natural::from(123u32).is_power_of_two(), false);
    ///     assert_eq!(Natural::from(0x80u32).is_power_of_two(), true);
    ///     assert_eq!(Natural::trillion().is_power_of_two(), false);
    ///     assert_eq!(Natural::from_str("1099511627776").unwrap().is_power_of_two(), true);
    /// }
    /// ```
    fn is_power_of_two(&self) -> bool {
        match *self {
            Small(small) => small.is_power_of_two(),
            Large(ref limbs) => limbs_is_power_of_two(limbs),
        }
    }
}