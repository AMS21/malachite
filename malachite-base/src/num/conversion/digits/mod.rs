/// This module provides traits for extracting digits from numbers and constructing numbers from
/// digits.
///
/// Here are usage examples of the macro-generated functions:
///
/// # to_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(0u8.to_digits_asc(&6u64), &[]);
/// assert_eq!(2u16.to_digits_asc(&6u32), &[2]);
/// assert_eq!(123456u32.to_digits_asc(&3u16), &[0, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2]);
/// ```
///
/// # to_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(0u8.to_digits_asc(&6u64), &[]);
/// assert_eq!(2u16.to_digits_asc(&6u32), &[2]);
/// assert_eq!(123456u32.to_digits_desc(&3u16), &[2, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0]);
/// ```
///
/// # from_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(u8::from_digits_asc(&64, [0u64, 0, 0].iter().cloned()), 0);
/// assert_eq!(
///     u32::from_digits_asc(&3, [0u64, 1, 1, 0, 0, 1, 1, 2, 0, 0, 2].iter().cloned()),
///     123456
/// );
/// assert_eq!(u32::from_digits_asc(&8, [3u16, 7, 1].iter().cloned()), 123);
/// ```
///
/// # from_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::Digits;
///
/// assert_eq!(u8::from_digits_desc(&64, [0u64, 0, 0].iter().cloned()), 0);
/// assert_eq!(
///     u32::from_digits_desc(&3, [2u64, 0, 0, 2, 1, 1, 0, 0, 1, 1, 0].iter().cloned()),
///     123456
/// );
/// assert_eq!(u32::from_digits_desc(&8, [1u16, 7, 3].iter().cloned()), 123);
/// ```
pub mod general_digits;
/// This module provides a double-ended iterator for iterating over a number's digits, if the base
/// is a power of two.
///
/// Here are usage examples of the macro-generated functions:
///
/// # power_of_two_digits
/// ```
/// extern crate itertools;
///
/// use itertools::Itertools;
///
/// use malachite_base::num::conversion::digits::power_of_two_digit_iterable::*;
/// use malachite_base::num::conversion::traits::PowerOfTwoDigitIterable;
///
/// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
/// assert!(digits.next().is_none());
///
/// // 107 = 1101011b
/// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
/// assert_eq!(digits.collect_vec(), &[3, 2, 2, 1]);
///
/// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(0u8, 2);
/// assert!(digits.next_back().is_none());
///
/// // 107 = 1101011b
/// let mut digits = PowerOfTwoDigitIterable::<u8>::power_of_two_digits(107u32, 2);
/// assert_eq!(digits.rev().collect_vec(), &[1, 2, 2, 3]);
/// ```
pub mod power_of_two_digit_iterable;
/// This module provides traits for extracting digits from numbers and constructing numbers from
/// digits, where the base is a power of two.
///
/// Here are usage examples of the macro-generated functions:
///
/// # to_power_of_two_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&0u8, 6), &[]);
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_asc(&2u16, 6), &[2]);
/// // 123_10 = 173_8
/// assert_eq!(PowerOfTwoDigits::<u16>::to_power_of_two_digits_asc(&123u32, 3), &[3, 7, 1]);
/// ```
///
/// # to_power_of_two_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&0u8, 6), &[]);
/// assert_eq!(PowerOfTwoDigits::<u64>::to_power_of_two_digits_desc(&2u16, 6), &[2]);
/// // 123_10 = 173_8
/// assert_eq!(PowerOfTwoDigits::<u16>::to_power_of_two_digits_desc(&123u32, 3), &[1, 7, 3]);
/// ```
///
/// # from_power_of_two_digits_asc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(u8::from_power_of_two_digits_asc(6, [0u64, 0, 0].iter().cloned()), 0);
/// assert_eq!(u16::from_power_of_two_digits_asc(6, [2u64, 0].iter().cloned()), 2);
/// assert_eq!(u32::from_power_of_two_digits_asc(3, [3u16, 7, 1].iter().cloned()), 123);
/// ```
///
/// # from_power_of_two_digits_desc
/// ```
/// use malachite_base::num::conversion::traits::PowerOfTwoDigits;
///
/// assert_eq!(u8::from_power_of_two_digits_desc(6, [0u64, 0, 0].iter().cloned()), 0);
/// assert_eq!(u16::from_power_of_two_digits_desc(6, [0u64, 2].iter().cloned()), 2);
/// assert_eq!(u32::from_power_of_two_digits_desc(3, [1u16, 7, 3].iter().cloned()), 123);
/// ```
pub mod power_of_two_digits;