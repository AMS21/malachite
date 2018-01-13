use integer::Integer;
use malachite_base::traits::AbsAssign;
use natural::Natural;

impl Integer {
    /// Finds the absolute value of an `Integer`, taking the `Integer` by value.
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
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.abs().to_string(), "0");
    ///     assert_eq!(Integer::from(123).abs().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).abs().to_string(), "123");
    /// }
    /// ```
    pub fn abs(mut self) -> Integer {
        self.sign = true;
        self
    }

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.abs_ref().to_string(), "0");
    ///     assert_eq!(Integer::from(123).abs_ref().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).abs_ref().to_string(), "123");
    /// }
    /// ```
    pub fn abs_ref(&self) -> Integer {
        Integer {
            sign: true,
            abs: self.abs.clone(),
        }
    }

    /// Finds the absolute value of an `Integer`, taking the `Integer` by value and converting the
    /// result to a `Natural`.
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
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.natural_abs().to_string(), "0");
    ///     assert_eq!(Integer::from(123).natural_abs().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).natural_abs().to_string(), "123");
    /// }
    /// ```
    pub fn natural_abs(self) -> Natural {
        self.abs
    }

    /// Finds the absolute value of an `Integer`, taking the `Integer` by reference and converting
    /// the result to a `Natural`.
    ///
    /// Time: worst case O(n)
    ///
    /// Additional memory: worst case O(n)
    ///
    /// where n = `self.significant_bits()`
    ///
    /// # Examples
    /// ```
    /// extern crate malachite_base;
    /// extern crate malachite_nz;
    ///
    /// use malachite_base::traits::Zero;
    /// use malachite_nz::integer::Integer;
    ///
    /// fn main() {
    ///     assert_eq!(Integer::ZERO.natural_abs_ref().to_string(), "0");
    ///     assert_eq!(Integer::from(123).natural_abs_ref().to_string(), "123");
    ///     assert_eq!(Integer::from(-123).natural_abs_ref().to_string(), "123");
    /// }
    /// ```
    pub fn natural_abs_ref(&self) -> Natural {
        self.abs.clone()
    }
}

/// Replaces an `Integer` with its absolute value.
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
/// use malachite_base::traits::{AbsAssign, Zero};
/// use malachite_nz::integer::Integer;
///
/// fn main() {
///     let mut x = Integer::ZERO;
///     x.abs_assign();
///     assert_eq!(x.to_string(), "0");
///
///     let mut x = Integer::from(123);
///     x.abs_assign();
///     assert_eq!(x.to_string(), "123");
///
///     let mut x = Integer::from(-123);
///     x.abs_assign();
///     assert_eq!(x.to_string(), "123");
/// }
/// ```
impl AbsAssign for Integer {
    fn abs_assign(&mut self) {
        self.sign = true;
    }
}