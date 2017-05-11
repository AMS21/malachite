use natural::Natural::{self, Small};

/// Converts a `u32` to a `Natural`.
///
/// # Example
/// ```
/// use malachite_native::natural::Natural;
///
/// assert_eq!(Natural::from(123).to_string(), "123");
/// ```
impl From<u32> for Natural {
    fn from(u: u32) -> Natural {
        Small(u)
    }
}