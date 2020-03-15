use std::str::FromStr;

use malachite_base::num::arithmetic::traits::{ModPowerOfTwo, NegModPowerOfTwo};
use malachite_base::num::basic::traits::{One, Zero};
use malachite_base::num::logic::integers::_assign_bits_naive;
use malachite_base::num::logic::traits::{BitBlockAccess, SignificantBits};
use malachite_nz::integer::logic::bit_block_access::limbs_neg_assign_bits;
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;

use malachite_test::common::test_properties;
use malachite_test::inputs::base::*;
use malachite_test::inputs::integer::{
    integers, natural_integers, negative_integers,
    quadruples_of_integer_small_unsigned_small_unsigned_and_natural_var_1,
    triples_of_integer_unsigned_and_natural,
};
use malachite_test::inputs::natural::{
    naturals, pairs_of_natural_and_small_unsigned,
    quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1,
};

fn verify_limbs_neg_assign_bits(limbs: &[Limb], start: u64, end: u64, bits: &[Limb], out: &[Limb]) {
    let old_n = -Natural::from_limbs_asc(limbs);
    let mut n = old_n.clone();
    let bits = Natural::from_limbs_asc(bits);
    n.assign_bits(start, end, &bits);
    let result = n;
    assert_eq!(-Natural::from_limbs_asc(out), result);
    let mut n = old_n.clone();
    _assign_bits_naive::<Integer, Natural>(&mut n, start, end, &bits);
    assert_eq!(n, result);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_neg_assign_bits() {
    let test = |limbs_in: &[Limb], start: u64, end: u64, bits: &[Limb], limbs_out: &[Limb]| {
        let mut limbs = limbs_in.to_vec();
        limbs_neg_assign_bits(&mut limbs, start, end, bits);
        assert_eq!(limbs, limbs_out);
        verify_limbs_neg_assign_bits(limbs_in, start, end, bits, limbs_out);
    };
    test(&[1], 0, 1, &[1], &[1]);
    test(&[1], 1, 2, &[1], &[1]);
    test(&[1], 0, 1, &[0, 1], &[2]);
    test(&[123], 64, 128, &[456], &[123, 0, 4294966839, 4294967295]);
    test(&[123], 80, 100, &[456], &[123, 0, 4265017344, 15]);
    test(
        &[123, 456],
        80,
        100,
        &[789, 321],
        &[123, 456, 4243193856, 15],
    );
    test(
        &[1619367413, 294928230],
        73,
        89,
        &[
            4211621339, 3627566573, 1208090001, 4045783696, 2932656682, 177881999, 898588654,
        ],
        &[1619367413, 294928230, 25446400],
    );
    test(
        &[
            1404969050, 495263765, 2378891263, 1299524786, 1654909014, 2724647948,
        ],
        21,
        32,
        &[
            3269073749, 1170977875, 2823122906, 144832001, 3738801070, 1107604886, 4260406413,
            1766163855, 592730267, 484513503, 1204041536, 3664297641,
        ],
        &[
            2505973850, 495263765, 2378891263, 1299524786, 1654909014, 2724647948,
        ],
    );
    test(
        &[
            4126931041, 1467617913, 1718397261, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
        ],
        27,
        77,
        &[977841009],
        &[
            1979447393, 4264409764, 1718403071, 904474857, 312429577, 2397873671, 3967827549,
            3842236128, 3414636734, 1846949256, 1999024107, 424639176,
        ],
    );
    test(&[123, 456], 0, 100, &[], &[0, 0, 0, 16]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_1() {
    let mut limbs = vec![123];
    limbs_neg_assign_bits(&mut limbs, 10, 5, &[456]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_2() {
    let mut limbs = vec![123];
    limbs_neg_assign_bits(&mut limbs, 10, 10, &[456]);
}

#[test]
#[should_panic]
fn limbs_neg_assign_bits_fail_3() {
    let mut limbs = vec![];
    limbs_neg_assign_bits(&mut limbs, 10, 10, &[456]);
}

#[test]
fn test_assign_bits() {
    let test = |u, start, end, v, out| {
        let mut n = Integer::from_str(u).unwrap();
        n.assign_bits(start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Integer::from_str(out).unwrap());
        let mut n = Integer::from_str(u).unwrap();
        _assign_bits_naive(&mut n, start, end, &Natural::from_str(v).unwrap());
        assert_eq!(n, Integer::from_str(out).unwrap());
    };
    test("123", 10, 10, "456", "123");
    test("123", 5, 7, "456", "27");
    test("123", 64, 128, "456", "8411715297611555537019");
    test("123", 80, 100, "456", "551270173744270903666016379");
    test(
        "1000000000000",
        80,
        100,
        "456",
        "551270173744271903666016256",
    );
    test(
        "456",
        80,
        100,
        "1000000000000",
        "401092572728463209067316249032",
    );
    test(
        "1000000000000",
        80,
        100,
        "2000000000000",
        "802185145456926419134632497152",
    );

    test("-123", 10, 10, "456", "-123");
    test("-123", 5, 7, "456", "-123");
    test(
        "-123",
        64,
        128,
        "456",
        "-340282366920938455033212565746503123067",
    );
    test("-123", 80, 100, "456", "-1267098121128665515963862483067");
    test(
        "-1000000000000",
        80,
        100,
        "456",
        "-1267098121128665516963862482944",
    );
    test(
        "-456",
        80,
        100,
        "1000000000000",
        "-866556818573946577800212251080",
    );
    test(
        "-1000000000000",
        80,
        100,
        "2000000000000",
        "-465464245845483369732896002048",
    );
    test("-123", 0, 100, "0", "-1267650600228229401496703205376");
}

#[test]
#[should_panic]
fn assign_bits_fail() {
    let mut n = Integer::from(123u32);
    n.assign_bits(10, 5, &Natural::from(456u32));
}

#[test]
fn limbs_neg_assign_bits_properties() {
    test_properties(
        quadruples_of_unsigned_vec_small_unsigned_small_unsigned_and_unsigned_vec_var_2,
        |&(ref limbs_in, start, end, ref bits)| {
            let mut limbs = limbs_in.to_vec();
            limbs_neg_assign_bits(&mut limbs, start, end, bits);
            verify_limbs_neg_assign_bits(limbs_in, start, end, bits, &limbs);
        },
    );
}

#[test]
fn assign_bits_properties() {
    test_properties(
        quadruples_of_integer_small_unsigned_small_unsigned_and_natural_var_1,
        |&(ref n, start, end, ref bits)| {
            let old_n = n;
            let mut n = old_n.clone();
            n.assign_bits(start, end, bits);
            let result = n;
            let mut n = old_n.clone();
            _assign_bits_naive(&mut n, start, end, bits);
            assert_eq!(n, result);
            n.assign_bits(start, end, bits);
            assert_eq!(n, result);
            let bits_width = end - start;
            assert_eq!(n.get_bits(start, end), bits.mod_power_of_two(bits_width));
            let mut n = !old_n;
            //TODO use sub_mod_power_of_two
            let mut not_bits = bits.neg_mod_power_of_two(bits_width);
            if not_bits == 0 {
                not_bits = (Natural::ONE << bits_width) - Natural::ONE;
            } else {
                not_bits -= Natural::ONE;
            }
            n.assign_bits(start, end, &not_bits);
            assert_eq!(!n, result);
        },
    );

    test_properties(
        triples_of_integer_unsigned_and_natural,
        |&(ref n, start, ref bits)| {
            let old_n = n;
            let mut n = old_n.clone();
            n.assign_bits(start, start, bits);
            assert_eq!(n, *old_n);
        },
    );

    test_properties(pairs_of_natural_and_small_unsigned, |&(ref bits, start)| {
        let mut n = Integer::ZERO;
        n.assign_bits(start, start + bits.significant_bits(), bits);
        assert_eq!(n, bits << start);
    });

    test_properties(integers, |n| {
        let old_n = n;
        let mut n = old_n.clone();
        let significant_bits = old_n.significant_bits();
        n.assign_bits(
            0,
            significant_bits,
            &old_n.mod_power_of_two(significant_bits),
        );
        assert_eq!(n, *old_n);
    });

    test_properties(natural_integers, |n| {
        let old_n = n;
        let mut n = old_n.clone();
        n.assign_bits(0, old_n.significant_bits(), &Natural::ZERO);
        assert_eq!(n, 0);
    });

    test_properties(negative_integers, |n| {
        let old_n = n;
        let mut n = old_n.clone();
        let significant_bits = old_n.significant_bits();
        n.assign_bits(
            0,
            significant_bits,
            &((Natural::ONE << significant_bits) - Natural::ONE),
        );
        assert_eq!(n, -1);

        //TODO use sub_mod_power_of_two to assign bits to -1
    });

    test_properties(naturals, |n| {
        let old_n = n;
        let mut n = Integer::ZERO;
        n.assign_bits(0, old_n.significant_bits(), old_n);
        assert_eq!(n, *old_n);
    });

    test_properties(
        quadruples_of_natural_small_unsigned_small_unsigned_and_natural_var_1,
        |&(ref n, start, end, ref bits)| {
            let old_n = n;
            let mut n = old_n.clone();
            n.assign_bits(start, end, bits);
            let result = n;
            let mut n = Integer::from(old_n);
            n.assign_bits(start, end, bits);
            assert_eq!(n, result);
        },
    );
}