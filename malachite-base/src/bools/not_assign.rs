use num::logic::traits::NotAssign;

impl NotAssign for bool {
    /// Replaces a `bool` with its negation.
    ///
    /// Time: worst case O(1)
    ///
    /// Additional memory: worst case O(1)
    ///
    /// # Examples
    /// ```
    /// use malachite_base::num::logic::traits::NotAssign;
    ///
    /// let mut b = false;
    /// b.not_assign();
    /// assert_eq!(b, true);
    ///
    /// let mut b = true;
    /// b.not_assign();
    /// assert_eq!(b, false);
    /// ```
    #[inline]
    fn not_assign(&mut self) {
        *self = !*self
    }
}