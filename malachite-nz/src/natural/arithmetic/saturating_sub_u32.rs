use malachite_base::num::{CheckedSub, SaturatingSub, SaturatingSubAssign, Zero};
use natural::Natural;

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by value and returning zero if the
/// `u32` is greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((Natural::from(123u32).saturating_sub(123)).to_string(), "0");
///     assert_eq!((Natural::from(123u32).saturating_sub(0)).to_string(), "123");
///     assert_eq!((Natural::from(456u32).saturating_sub(123)).to_string(), "333");
///     assert_eq!((Natural::from(123u32).saturating_sub(456)).to_string(), "0");
///     assert_eq!((Natural::trillion().saturating_sub(123)).to_string(), "999999999877");
/// }
/// ```
impl SaturatingSub<u32> for Natural {
    type Output = Natural;

    fn saturating_sub(self, other: u32) -> Natural {
        self.checked_sub(other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `u32` from a `Natural`, taking the `Natural` by reference and returning zero if the
/// `u32` is greater than the `Natural`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(n)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!((&Natural::from(123u32).saturating_sub(123)).to_string(), "0");
///     assert_eq!((&Natural::from(123u32).saturating_sub(0)).to_string(), "123");
///     assert_eq!((&Natural::from(456u32).saturating_sub(123)).to_string(), "333");
///     assert_eq!((&Natural::from(123u32).saturating_sub(456)).to_string(), "0");
///     assert_eq!((&Natural::trillion().saturating_sub(123)).to_string(), "999999999877");
/// }
/// ```
impl<'a> SaturatingSub<u32> for &'a Natural {
    type Output = Natural;

    fn saturating_sub(self, other: u32) -> Natural {
        self.checked_sub(other).unwrap_or(Natural::ZERO)
    }
}

/// Subtracts a `u32` from a `Natural` in place. Sets the `Natural` to zero if it is smaller than
/// the `u32`.
///
/// Time: worst case O(n)
///
/// Additional memory: worst case O(1)
///
/// where n = `self.significant_bits()`
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Example
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSubAssign, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = Natural::from(15u32);
///     x.saturating_sub_assign(1);
///     x.saturating_sub_assign(2);
///     x.saturating_sub_assign(3);
///     x.saturating_sub_assign(4);
///     assert_eq!(x.to_string(), "5");
///     x.saturating_sub_assign(10);
///     assert_eq!(x.to_string(), "0");
/// }
/// ```
impl SaturatingSubAssign<u32> for Natural {
    fn saturating_sub_assign(&mut self, other: u32) {
        if self.sub_assign_u32_no_panic(other) {
            *self = Natural::ZERO;
        }
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by value, returning zero if the
/// `Natural` is greater than the `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123.saturating_sub(Natural::from(123u32)), 0);
///     assert_eq!(123.saturating_sub(Natural::ZERO), 123);
///     assert_eq!(456.saturating_sub(Natural::from(123u32)), 333);
///     assert_eq!(123.saturating_sub(Natural::from(456u32)), 0);
/// }
/// ```
impl SaturatingSub<Natural> for u32 {
    type Output = u32;

    fn saturating_sub(self, other: Natural) -> u32 {
        CheckedSub::checked_sub(self, &other).unwrap_or(0)
    }
}

/// Subtracts a `Natural` from a `u32`, taking the `Natural` by reference, returning zero if the
/// `Natural` is greater than the `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSub, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     assert_eq!(123.saturating_sub(&Natural::from(123u32)), 0);
///     assert_eq!(123.saturating_sub(&Natural::ZERO), 123);
///     assert_eq!(456.saturating_sub(&Natural::from(123u32)), 333);
///     assert_eq!(123.saturating_sub(&Natural::from(456u32)), 0);
/// }
/// ```
impl<'a> SaturatingSub<&'a Natural> for u32 {
    type Output = u32;

    fn saturating_sub(self, other: &'a Natural) -> u32 {
        CheckedSub::checked_sub(self, other).unwrap_or(0)
    }
}

/// Subtracts a `Natural` from a `u32` in place, taking the `Natural` by value, returning zero if
/// the `Natural` is greater than the `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSubAssign, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = 123;
///     x.saturating_sub_assign(Natural::from(123u32));
///     assert_eq!(x, 0);
///
///     let mut x = 123;
///     x.saturating_sub_assign(Natural::ZERO);
///     assert_eq!(x, 123);
///
///     let mut x = 456;
///     x.saturating_sub_assign(Natural::from(123u32));
///     assert_eq!(x, 333);
///
///     let mut x = 123;
///     x.saturating_sub_assign(Natural::from(456u32));
///     assert_eq!(x, 0);
/// }
/// ```
impl SaturatingSubAssign<Natural> for u32 {
    fn saturating_sub_assign(&mut self, other: Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}

/// Subtracts a `Natural` from a `u32` in place, taking the `Natural` by reference, returning zero
/// if the `Natural` is greater than the `u32`.
///
/// Time: worst case O(1)
///
/// Additional memory: worst case O(1)
///
/// # Panics
/// Panics if `other` is greater than `self`.
///
/// # Examples
/// ```
/// extern crate malachite_base;
/// extern crate malachite_nz;
///
/// use malachite_base::num::{SaturatingSubAssign, Zero};
/// use malachite_nz::natural::Natural;
///
/// fn main() {
///     let mut x = 123;
///     x.saturating_sub_assign(&Natural::from(123u32));
///     assert_eq!(x, 0);
///
///     let mut x = 123;
///     x.saturating_sub_assign(&Natural::ZERO);
///     assert_eq!(x, 123);
///
///     let mut x = 456;
///     x.saturating_sub_assign(&Natural::from(123u32));
///     assert_eq!(x, 333);
///
///     let mut x = 123;
///     x.saturating_sub_assign(&Natural::from(456u32));
///     assert_eq!(x, 0);
/// }
/// ```
impl<'a> SaturatingSubAssign<&'a Natural> for u32 {
    fn saturating_sub_assign(&mut self, other: &'a Natural) {
        *self = SaturatingSub::saturating_sub(*self, other);
    }
}
