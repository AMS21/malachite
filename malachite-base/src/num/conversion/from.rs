use std::num::ParseIntError;

use comparison::traits::{Max, Min};
use num::basic::traits::Zero;
use num::conversion::traits::{
    CheckedFrom, ConvertibleFrom, ExactFrom, FromStrRadix, OverflowingFrom, SaturatingFrom,
    WrappingFrom,
};

/// This macro defines conversions from a type to itself.
macro_rules! identity_conversion {
    ($t:ty) => {
        impl CheckedFrom<$t> for $t {
            /// Converts a value to the value's type. Since this conversion is always valid and
            /// always leaves the value unchanged, `None` is never returned.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::CheckedFrom;
            ///
            /// assert_eq!(u8::checked_from(123u8), Some(123));
            /// assert_eq!(i32::checked_from(-5i32), Some(-5));
            /// ```
            #[inline]
            fn checked_from(value: $t) -> Option<$t> {
                Some(value)
            }
        }

        impl WrappingFrom<$t> for $t {
            /// Converts a value to the value's type. This conversion is always valid and always
            /// leaves the value unchanged.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::WrappingFrom;
            ///
            /// assert_eq!(u8::wrapping_from(123u8), 123);
            /// assert_eq!(i32::wrapping_from(-5i32), -5);
            /// ```
            #[inline]
            fn wrapping_from(value: $t) -> $t {
                value
            }
        }

        impl SaturatingFrom<$t> for $t {
            /// Converts a value to the value's type. This conversion is always valid and always
            /// leaves the value unchanged.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::SaturatingFrom;
            ///
            /// assert_eq!(u8::saturating_from(123u8), 123);
            /// assert_eq!(i32::saturating_from(-5i32), -5);
            /// ```
            #[inline]
            fn saturating_from(value: $t) -> $t {
                value
            }
        }

        impl OverflowingFrom<$t> for $t {
            /// Converts a value to the value's type. Since this conversion is always valid and
            /// always leaves the value unchanged, the second component of the result is always
            /// false (no overflow).
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::OverflowingFrom;
            ///
            /// assert_eq!(u8::overflowing_from(123u8), (123, false));
            /// assert_eq!(i32::overflowing_from(-5i32), (-5, false));
            /// ```
            #[inline]
            fn overflowing_from(value: $t) -> ($t, bool) {
                (value, false)
            }
        }

        impl ConvertibleFrom<$t> for $t {
            /// Checks whether a value is convertible to its own type. The result is always `true`.
            ///
            /// Time: worst case O(1)
            ///
            /// Additional memory: worst case O(1)
            ///
            /// # Examples
            /// ```
            /// use malachite_base::num::conversion::traits::ConvertibleFrom;
            ///
            /// assert!(u8::convertible_from(123u8));
            /// assert!(i32::convertible_from(-5i32));
            /// ```
            #[inline]
            fn convertible_from(_: $t) -> bool {
                true
            }
        }
    };
}

fn _checked_from_lossless<A, B: From<A>>(value: A) -> Option<B> {
    Some(B::from(value))
}

/// This macro defines conversions from type $a to type $b, where every value of type $a is
/// representable by a value of type $b.
macro_rules! lossless_conversion {
    ($a:ty, $b:ident) => {
        /// Converts a value to another type. Since this conversion is always lossless and leaves
        /// the value unchanged, `None` is never returned.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::CheckedFrom;
        ///
        /// assert_eq!(u16::checked_from(123u8), Some(123));
        /// assert_eq!(i64::checked_from(-5i32), Some(-5));
        /// ```
        impl CheckedFrom<$a> for $b {
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                _checked_from_lossless(value)
            }
        }

        /// Converts a value to another type. This conversion is always valid and always leaves the
        /// value unchanged.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::WrappingFrom;
        ///
        /// assert_eq!(u16::wrapping_from(123u8), 123);
        /// assert_eq!(i64::wrapping_from(-5i32), -5);
        /// ```
        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        /// Converts a value to another type. This conversion is always valid and always leaves the
        /// value unchanged.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::SaturatingFrom;
        ///
        /// assert_eq!(u16::saturating_from(123u8), 123);
        /// assert_eq!(i64::saturating_from(-5i32), -5);
        /// ```
        impl SaturatingFrom<$a> for $b {
            #[inline]
            fn saturating_from(value: $a) -> $b {
                $b::from(value)
            }
        }

        /// Converts a value to the value's type. Since this conversion is always valid and
        /// always leaves the value unchanged, the second component of the result is always
        /// false (no overflow).
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::OverflowingFrom;
        ///
        /// assert_eq!(u16::overflowing_from(123u8), (123, false));
        /// assert_eq!(i64::overflowing_from(-5i32), (-5, false));
        /// ```
        impl OverflowingFrom<$a> for $b {
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                ($b::from(value), false)
            }
        }

        /// Checks whether a value is convertible to a different type. The result is always `true`.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::ConvertibleFrom;
        ///
        /// assert!(u16::convertible_from(123u8));
        /// assert!(i64::convertible_from(-5i32));
        /// ```
        impl ConvertibleFrom<$a> for $b {
            #[inline]
            fn convertible_from(_: $a) -> bool {
                true
            }
        }
    };
}

fn _checked_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> Option<B> {
    let result = B::wrapping_from(value);
    if (result >= B::ZERO) == (value >= A::ZERO) && A::wrapping_from(result) == value {
        Some(result)
    } else {
        None
    }
}

fn _saturating_from_lossy<A: CheckedFrom<B> + Ord, B: Max + Min + WrappingFrom<A>>(value: A) -> B {
    if let Some(b_max) = A::checked_from(B::MAX) {
        if value >= b_max {
            return B::MAX;
        }
    }
    if let Some(b_min) = A::checked_from(B::MIN) {
        if value <= b_min {
            return B::MIN;
        }
    }
    B::wrapping_from(value)
}

fn _overflowing_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> (B, bool) {
    let result = B::wrapping_from(value);
    (
        result,
        (result >= B::ZERO) != (value >= A::ZERO) || A::wrapping_from(result) != value,
    )
}

fn _convertible_from_lossy<
    A: Copy + Ord + WrappingFrom<B> + Zero,
    B: Copy + Ord + WrappingFrom<A> + Zero,
>(
    value: A,
) -> bool {
    let result = B::wrapping_from(value);
    (result >= B::ZERO) == (value >= A::ZERO) && A::wrapping_from(result) == value
}

/// This macro defines conversions from type $a to type $b, where not every value of type $a is
/// representable by a value of type $b.
macro_rules! lossy_conversion {
    ($a:ident, $b:ident) => {
        /// Converts a value to another type. If the value cannot be represented in the new type,
        /// `None` is returned.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::CheckedFrom;
        ///
        /// assert_eq!(u32::checked_from(5u64), Some(5));
        /// assert_eq!(u8::checked_from(1_000u16), None);
        /// assert_eq!(u32::checked_from(-5i32), None);
        /// assert_eq!(i32::checked_from(3_000_000_000u32), None);
        /// assert_eq!(i8::checked_from(-1_000i16), None);
        /// ```
        impl CheckedFrom<$a> for $b {
            #[inline]
            fn checked_from(value: $a) -> Option<$b> {
                _checked_from_lossy(value)
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type, it
        /// is wrapped.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::WrappingFrom;
        ///
        /// assert_eq!(u32::wrapping_from(5u64), 5);
        /// assert_eq!(u8::wrapping_from(1_000u16), 232);
        /// assert_eq!(u32::wrapping_from(-5i32), 4_294_967_291);
        /// assert_eq!(i32::wrapping_from(3_000_000_000u32), -1_294_967_296);
        /// assert_eq!(i8::wrapping_from(-1_000i16), 24);
        /// ```
        #[allow(clippy::cast_lossless)]
        impl WrappingFrom<$a> for $b {
            #[inline]
            fn wrapping_from(value: $a) -> $b {
                value as $b
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type,
        /// the maximum or minimum value of the new type, whichever is closer, is returned.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::SaturatingFrom;
        ///
        /// assert_eq!(u32::saturating_from(5u64), 5);
        /// assert_eq!(u8::saturating_from(1_000u16), 255);
        /// assert_eq!(u32::saturating_from(-5i32), 0);
        /// assert_eq!(i32::saturating_from(3_000_000_000u32), 2_147_483_647);
        /// assert_eq!(i8::saturating_from(-1_000i16), -128);
        /// ```
        impl SaturatingFrom<$a> for $b {
            #[inline]
            fn saturating_from(value: $a) -> $b {
                _saturating_from_lossy(value)
            }
        }

        /// Converts a value to another type. If the value cannot be represented in the new type, it
        /// is wrapped. The second component of the result indicates whether overflow occurred.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::OverflowingFrom;
        ///
        /// assert_eq!(u32::overflowing_from(5u64), (5, false));
        /// assert_eq!(u8::overflowing_from(1_000u16), (232, true));
        /// assert_eq!(u32::overflowing_from(-5i32), (4_294_967_291, true));
        /// assert_eq!(i32::overflowing_from(3_000_000_000u32), (-1_294_967_296, true));
        /// assert_eq!(i8::overflowing_from(-1_000i16), (24, true));
        /// ```
        impl OverflowingFrom<$a> for $b {
            #[inline]
            fn overflowing_from(value: $a) -> ($b, bool) {
                _overflowing_from_lossy(value)
            }
        }

        /// Determines whether a value is convertible to a different type.
        ///
        /// Time: worst case O(1)
        ///
        /// Additional memory: worst case O(1)
        ///
        /// # Examples
        /// ```
        /// use malachite_base::num::conversion::traits::ConvertibleFrom;
        ///
        /// assert_eq!(u32::convertible_from(5u64), true);
        /// assert_eq!(u8::convertible_from(1_000u16), false);
        /// assert_eq!(u32::convertible_from(-5i32), false);
        /// assert_eq!(i32::convertible_from(3_000_000_000u32), false);
        /// assert_eq!(i8::convertible_from(-1_000i16), false);
        /// ```
        impl ConvertibleFrom<$a> for $b {
            #[inline]
            fn convertible_from(value: $a) -> bool {
                _convertible_from_lossy::<$a, $b>(value)
            }
        }
    };
}

/// This macro defines conversions from type $a to type $b, where the set of values representable by
/// type $a is a proper subset of the set of values representable by type $b.
macro_rules! proper_subset_conversion {
    ($a:ident, $b:ident) => {
        lossless_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

/// This macro defines conversions from type $a to type $b, where the set of values representable by
/// type $a is neither a subset nor a superset of the set of values representable by type $b.
macro_rules! no_containment_conversion {
    ($a:ident, $b:ident) => {
        lossy_conversion!($a, $b);
        lossy_conversion!($b, $a);
    };
}

apply_to_primitive_ints!(identity_conversion);

proper_subset_conversion!(u8, u16);
proper_subset_conversion!(u8, u32);
proper_subset_conversion!(u8, u64);
proper_subset_conversion!(u8, u128);
proper_subset_conversion!(u8, usize);
proper_subset_conversion!(u8, i16);
proper_subset_conversion!(u8, i32);
proper_subset_conversion!(u8, i64);
proper_subset_conversion!(u8, i128);
proper_subset_conversion!(u8, isize);
proper_subset_conversion!(u16, u32);
proper_subset_conversion!(u16, u64);
proper_subset_conversion!(u16, u128);
proper_subset_conversion!(u16, usize);
proper_subset_conversion!(u16, i32);
proper_subset_conversion!(u16, i64);
proper_subset_conversion!(u16, i128);
proper_subset_conversion!(u32, u64);
proper_subset_conversion!(u32, u128);
proper_subset_conversion!(u32, i64);
proper_subset_conversion!(u32, i128);
proper_subset_conversion!(u64, u128);
proper_subset_conversion!(u64, i128);
proper_subset_conversion!(i8, i16);
proper_subset_conversion!(i8, i32);
proper_subset_conversion!(i8, i64);
proper_subset_conversion!(i8, i128);
proper_subset_conversion!(i8, isize);
proper_subset_conversion!(i16, i32);
proper_subset_conversion!(i16, i64);
proper_subset_conversion!(i16, i128);
proper_subset_conversion!(i16, isize);
proper_subset_conversion!(i32, i64);
proper_subset_conversion!(i32, i128);
proper_subset_conversion!(i64, i128);

no_containment_conversion!(u8, i8);
no_containment_conversion!(u16, i8);
no_containment_conversion!(u16, i16);
no_containment_conversion!(u16, isize);
no_containment_conversion!(u32, usize);
no_containment_conversion!(u32, i8);
no_containment_conversion!(u32, i16);
no_containment_conversion!(u32, i32);
no_containment_conversion!(u32, isize);
no_containment_conversion!(u64, usize);
no_containment_conversion!(u64, i8);
no_containment_conversion!(u64, i16);
no_containment_conversion!(u64, i32);
no_containment_conversion!(u64, i64);
no_containment_conversion!(u64, isize);
no_containment_conversion!(u128, usize);
no_containment_conversion!(u128, i8);
no_containment_conversion!(u128, i16);
no_containment_conversion!(u128, i32);
no_containment_conversion!(u128, i64);
no_containment_conversion!(u128, i128);
no_containment_conversion!(u128, isize);
no_containment_conversion!(usize, i8);
no_containment_conversion!(usize, i16);
no_containment_conversion!(usize, i32);
no_containment_conversion!(usize, i64);
no_containment_conversion!(usize, i128);
no_containment_conversion!(usize, isize);
no_containment_conversion!(i32, isize);
no_containment_conversion!(i64, isize);
no_containment_conversion!(i128, isize);

macro_rules! impl_conversion_traits {
    ($t:ident) => {
        impl FromStrRadix for $t {
            #[inline]
            fn from_str_radix(src: &str, radix: u64) -> Result<Self, ParseIntError> {
                $t::from_str_radix(src, u32::exact_from(radix))
            }
        }
    };
}
apply_to_primitive_ints!(impl_conversion_traits);
