use std::str::FromStr;

use malachite_base::num::logic::integers::{_to_bits_asc_alt, _to_bits_desc_alt};
use malachite_base::num::logic::traits::{BitConvertible, BitIterable};
use malachite_nz::integer::logic::bit_convertible::{
    bits_slice_to_twos_complement_bits_negative, bits_to_twos_complement_bits_non_negative,
    bits_vec_to_twos_complement_bits_negative,
};
use malachite_nz::integer::Integer;
use malachite_nz::platform::SignedLimb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::{signeds, vecs_of_bool, vecs_of_bool_var_1};
use malachite_test::inputs::integer::integers;
use malachite_test::integer::logic::to_bits::{_to_bits_asc_naive, _to_bits_desc_naive};

#[test]
fn test_bits_to_twos_complement_bits_non_negative() {
    let test = |bits: &[bool], out_bits: &[bool]| {
        let mut mut_bits = bits.to_vec();
        bits_to_twos_complement_bits_non_negative(&mut mut_bits);
        assert_eq!(mut_bits, out_bits);
    };
    test(&[], &[]);
    test(&[false, true, false], &[false, true, false]);
    test(&[true, false, true], &[true, false, true, false]);
}

#[test]
fn test_bits_slice_to_twos_complement_bits_negative() {
    let test = |bits: &[bool], out_bits: &[bool], carry: bool| {
        let mut mut_bits = bits.to_vec();
        assert_eq!(
            bits_slice_to_twos_complement_bits_negative(&mut mut_bits),
            carry
        );
        assert_eq!(mut_bits, out_bits);
    };
    test(&[], &[], true);
    test(&[true, false, true], &[true, true, false], false);
    test(&[false, false, false], &[false, false, false], true);
}

#[test]
fn test_bits_vec_to_twos_complement_bits_negative() {
    let test = |bits: &[bool], out_bits: &[bool]| {
        let mut mut_bits = bits.to_vec();
        bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
        assert_eq!(mut_bits, out_bits);
    };
    test(&[true, false, false], &[true, true, true]);
    test(&[true, false, true], &[true, true, false, true]);
}

#[test]
#[should_panic]
fn bits_vec_to_twos_complement_bits_negative_fail() {
    let mut mut_bits = vec![false, false];
    bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
}

#[test]
fn test_to_bits_asc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.bits().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_asc(), out);
        assert_eq!(_to_bits_asc_naive(&n), out);
    };
    test("0", vec![]);
    test("1", vec![true, false]);
    test("-1", vec![true]);
    test("6", vec![false, true, true, false]);
    test("-6", vec![false, true, false, true]);
    test(
        "105",
        vec![true, false, false, true, false, true, true, false],
    );
    test(
        "-105",
        vec![true, true, true, false, true, false, false, true],
    );
    test(
        "1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, false, false, false, true, false, true, false, false, true, false, true, false,
            false, true, false, true, false, true, true, false, false, false, true, false, true,
            true, true, false,
        ],
    );
    test(
        "-1000000000000",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            true, true, true, true, false, true, false, true, true, false, true, false, true, true,
            false, true, false, true, false, false, true, true, true, false, true, false, false,
            false, true,
        ],
    );
    test(
        "4294967295",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, false,
        ],
    );
    test(
        "-4294967295",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true, false,
        ],
    );
    test(
        "-4294967296",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, false,
        ],
    );
    test(
        "-18446744073709551615",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true, false,
        ],
    );
    test(
        "-18446744073709551616",
        vec![
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "-10725406948920873257320529212268773241779870075",
        vec![
            true, false, true, false, false, false, false, true, false, true, true, false, false,
            true, false, true, false, false, true, false, false, true, false, false, true, true,
            true, true, false, false, false, false, true, true, true, true, false, false, true,
            false, true, false, true, true, false, true, false, false, true, false, false, false,
            true, false, true, true, true, true, false, false, true, true, false, false, false,
            false, true, true, true, false, false, true, true, true, true, true, false, false,
            true, true, true, true, true, false, false, false, true, false, false, true, true,
            false, false, false, false, true, true, false, false, true, true, false, true, false,
            true, true, true, true, false, true, true, false, true, false, true, true, true, false,
            false, true, true, false, true, true, false, false, true, true, true, false, true,
            true, true, false, true, false, false, true, true, true, false, false, false, false,
            true, true, true, true, true, false, false, false, false, true,
        ],
    );
}

#[test]
fn test_to_bits_desc() {
    let test = |n, out| {
        let n = Integer::from_str(n).unwrap();
        assert_eq!(n.bits().rev().collect::<Vec<bool>>(), out);
        assert_eq!(n.to_bits_desc(), out);
        assert_eq!(_to_bits_desc_naive(&n), out);
        assert_eq!(_to_bits_desc_alt(&n), out);
    };
    test("0", vec![]);
    test("1", vec![false, true]);
    test("-1", vec![true]);
    test("6", vec![false, true, true, false]);
    test("-6", vec![true, false, true, false]);
    test(
        "105",
        vec![false, true, true, false, true, false, false, true],
    );
    test(
        "-105",
        vec![true, false, false, true, false, true, true, true],
    );
    test(
        "1000000000000",
        vec![
            false, true, true, true, false, true, false, false, false, true, true, false, true,
            false, true, false, false, true, false, true, false, false, true, false, true, false,
            false, false, true, false, false, false, false, false, false, false, false, false,
            false, false, false,
        ],
    );
    test(
        "-1000000000000",
        vec![
            true, false, false, false, true, false, true, true, true, false, false, true, false,
            true, false, true, true, false, true, false, true, true, false, true, false, true,
            true, true, true, false, false, false, false, false, false, false, false, false, false,
            false, false,
        ],
    );
    test(
        "4294967295",
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true,
        ],
    );
    test(
        "-4294967295",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, true,
        ],
    );
    test(
        "4294967296",
        vec![
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "-4294967296",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false,
        ],
    );
    test(
        "18446744073709551615",
        vec![
            false, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true, true, true, true, true, true,
            true, true, true, true, true, true, true, true, true,
        ],
    );
    test(
        "-18446744073709551615",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, true,
        ],
    );
    test(
        "18446744073709551616",
        vec![
            false, true, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false,
        ],
    );
    test(
        "-18446744073709551616",
        vec![
            true, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false, false, false, false, false, false, false, false,
            false, false, false, false, false,
        ],
    );
}

#[test]
fn bits_to_twos_complement_bits_non_negative_properties() {
    test_properties(vecs_of_bool, |bits| {
        let mut mut_bits = bits.clone();
        bits_to_twos_complement_bits_non_negative(&mut mut_bits);
    });
}

#[test]
fn bits_slice_to_twos_complement_bits_negative_properties() {
    test_properties(vecs_of_bool, |bits| {
        let mut mut_bits = bits.clone();
        bits_slice_to_twos_complement_bits_negative(&mut mut_bits);
    });
}

#[test]
fn bits_vec_to_twos_complement_bits_negative_properties() {
    test_properties(vecs_of_bool_var_1, |bits| {
        let mut mut_bits = bits.clone();
        bits_vec_to_twos_complement_bits_negative(&mut mut_bits);
    });
}

#[test]
fn to_bits_asc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_bits_asc();
        assert_eq!(_to_bits_asc_naive(x), bits);
        assert_eq!(_to_bits_asc_alt(x), bits);
        assert_eq!(x.bits().collect::<Vec<bool>>(), bits);
        assert_eq!(Integer::from_bits_asc(&bits), *x);
        if *x != 0 {
            assert_eq!(*bits.last().unwrap(), *x < 0);
        }
        let bit_len = bits.len();
        if bit_len > 1 {
            assert_ne!(bits[bit_len - 1], bits[bit_len - 2]);
        }
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.to_bits_asc(), Integer::from(i).to_bits_asc());
    });
}

#[test]
fn to_bits_desc_properties() {
    test_properties(integers, |x| {
        let bits = x.to_bits_desc();
        assert_eq!(_to_bits_desc_naive(x), bits);
        assert_eq!(_to_bits_desc_alt(x), bits);
        assert_eq!(x.bits().rev().collect::<Vec<bool>>(), bits);
        assert_eq!(Integer::from_bits_desc(&bits), *x);
        if *x != 0 {
            assert_eq!(bits[0], *x < 0);
        }
        if bits.len() > 1 {
            assert_ne!(bits[0], bits[1]);
        }
    });

    test_properties(signeds::<SignedLimb>, |&i| {
        assert_eq!(i.to_bits_desc(), Integer::from(i).to_bits_desc());
    });
}