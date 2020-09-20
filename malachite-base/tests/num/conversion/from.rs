use std::fmt::Debug;

use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{
    FromOtherTypeSlice, JoinHalves, SplitInHalf, VecFromOtherType, VecFromOtherTypeSlice,
};

fn split_in_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: (T::Half, T::Half))
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.split_in_half(), out);
}

#[test]
pub fn test_split_in_half() {
    split_in_half_helper(0u64, (0u32, 0u32));
    split_in_half_helper(1u64, (0u32, 1u32));
    split_in_half_helper(u16::from(u8::MAX), (0, u8::MAX));
    split_in_half_helper(u16::from(u8::MAX) + 1, (1, 0));
    split_in_half_helper(u16::MAX, (u8::MAX, u8::MAX));
    split_in_half_helper(258u16, (1u8, 2u8));
    split_in_half_helper(0xabcd_1234u32, (0xabcd, 0x1234));
}

fn lower_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.lower_half(), out);
}

#[test]
pub fn test_lower_half() {
    lower_half_helper(0u64, 0u32);
    lower_half_helper(1u64, 1u32);
    lower_half_helper(u16::from(u8::MAX), u8::MAX);
    lower_half_helper(u16::from(u8::MAX) + 1, 0);
    lower_half_helper(u16::MAX, u8::MAX);
    lower_half_helper(258u16, 2u8);
    lower_half_helper(0xabcd_1234u32, 0x1234);
}

fn upper_half_helper<T: PrimitiveUnsigned + SplitInHalf>(n: T, out: T::Half)
where
    T::Half: PrimitiveUnsigned,
{
    assert_eq!(n.upper_half(), out);
}

#[test]
pub fn test_upper_half() {
    upper_half_helper(0u64, 0u32);
    upper_half_helper(1u64, 0u32);
    upper_half_helper(u16::from(u8::MAX), 0);
    upper_half_helper(u16::from(u8::MAX) + 1, 1);
    upper_half_helper(u16::MAX, u8::MAX);
    upper_half_helper(258u16, 1u8);
    upper_half_helper(0xabcd_1234u32, 0xabcd);
}

fn join_halves_helper<T: JoinHalves + PrimitiveUnsigned>(upper: T::Half, lower: T::Half, out: T) {
    assert_eq!(T::join_halves(upper, lower), out);
}

#[test]
pub fn test_join_halves() {
    join_halves_helper(0u32, 0u32, 0u64);
    join_halves_helper(0u32, 1u32, 1u64);
    join_halves_helper(0, u8::MAX, u16::from(u8::MAX));
    join_halves_helper(1, 0, u16::from(u8::MAX) + 1);
    join_halves_helper(u8::MAX, u8::MAX, u16::MAX);
    join_halves_helper(1, 2, 258u16);
    join_halves_helper(0xabcd, 0x1234, 0xabcd_1234u32);
}

#[test]
pub fn test_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Copy + Debug + Eq + FromOtherTypeSlice<T>>(slice: &[T], n: U) {
        assert_eq!(U::from_other_type_slice(slice), n);
    };
    test::<u32, u32>(&[], 0);
    test::<u32, u32>(&[123], 123);
    test::<u32, u32>(&[123, 456], 123);

    test::<u8, u16>(&[0xab], 0xab);
    test::<u8, u16>(&[0xab, 0xcd], 0xcdab);
    test::<u8, u16>(&[0xab, 0xcd, 0xef], 0xcdab);
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67],
        0x67_4523_01ef_cdab,
    );
    test::<u8, u64>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        0x8967_4523_01ef_cdab,
    );

    test::<u64, u32>(&[], 0);
    test::<u16, u8>(&[0xabcd, 0xef01], 0xcd);
    test::<u128, u8>(&[0x1234_5678_90ab_cdef_0123_4567_890a_bcde], 0xde);
}

#[test]
pub fn test_vec_from_other_type_slice() {
    fn test<T: Debug + Eq, U: Debug + Eq + VecFromOtherTypeSlice<T>>(slice: &[T], vec: &[U]) {
        assert_eq!(U::vec_from_other_type_slice(slice), vec);
    };
    test::<u32, u32>(&[123, 456], &[123, 456]);
    test::<u8, u16>(
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89, 0xff],
        &[0xcdab, 0x01ef, 0x4523, 0x8967, 0xff],
    );
    test::<u8, u16>(&[0xab], &[0xab]);
    test::<u16, u8>(
        &[0xcdab, 0x01ef, 0x4523, 0x8967],
        &[0xab, 0xcd, 0xef, 0x01, 0x23, 0x45, 0x67, 0x89],
    );
}

#[test]
pub fn test_vec_from_other_type() {
    fn test<T: Debug + Eq, U: Debug + Eq + VecFromOtherType<T>>(value: T, vec: &[U]) {
        assert_eq!(U::vec_from_other_type(value), vec);
    };
    test::<u32, u32>(123, &[123]);
    test::<u8, u16>(0xab, &[0xab]);
    test::<u16, u8>(0xcdab, &[0xab, 0xcd]);
}
