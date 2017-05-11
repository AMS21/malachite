use gmp_mpfr_sys::gmp;
use integer::Integer::{self, Large, Small};
use std::cmp::Ordering;

/// Returns the sign of `self`. Interpret the result as the result of a comparison to zero, so that
/// `Equal` means zero, `Greater` means positive, and `Less` means negative.
///
/// # Examples
/// ```
/// use malachite_gmp::integer::Integer;
/// use std::cmp::Ordering;
///
/// assert_eq!(Integer::from(0).sign(), Ordering::Equal);
/// assert_eq!(Integer::from(123).sign(), Ordering::Greater);
/// assert_eq!(Integer::from(-123).sign(), Ordering::Less);
/// ```
impl Integer {
    pub fn sign(&self) -> Ordering {
        match *self {
            Small(x) => x.cmp(&0),
            Large(x) => (unsafe { gmp::mpz_sgn(&x) }).cmp(&0),
        }
    }
}