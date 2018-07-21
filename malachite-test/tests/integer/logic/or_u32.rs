use common::test_properties;
use malachite_base::num::{NegativeOne, Zero};
use malachite_nz::integer::logic::or_u32::{
    limbs_neg_or_limb, limbs_neg_or_limb_in_place, limbs_neg_or_limb_to_out,
};
use malachite_nz::integer::Integer;
use malachite_nz::natural::Natural;
use malachite_test::common::{integer_to_rug_integer, rug_integer_to_integer};
use malachite_test::inputs::base::{
    pairs_of_u32_vec_and_u32_var_1, triples_of_u32_vec_u32_vec_and_u32_var_3, unsigneds,
};
use malachite_test::inputs::integer::{integers, pairs_of_integer_and_unsigned};
use malachite_test::integer::logic::or_u32::{integer_or_u32_alt_1, integer_or_u32_alt_2};
use rug::{self, Assign};
use std::str::FromStr;

#[test]
fn test_limbs_neg_or_limb() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        assert_eq!(limbs_neg_or_limb(limbs, limb), out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[98, 101, 102]);
    test(&[123, 456], 789, &[107, 456]);
    test(&[0, 0, 456], 789, &[0xffff_fceb, 0xffff_ffff, 455]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn limbs_neg_or_limb_fail() {
    limbs_neg_or_limb(&[0, 0, 0], 10);
}

#[test]
fn test_limbs_neg_or_limb_to_out() {
    let test = |limbs_out_before: &[u32], limbs_in: &[u32], limb: u32, limbs_out_after: &[u32]| {
        let mut limbs_out = limbs_out_before.to_vec();
        limbs_neg_or_limb_to_out(&mut limbs_out, limbs_in, limb);
        assert_eq!(limbs_out, limbs_out_after);
    };
    test(&[10, 10, 10, 10], &[6, 7], 0, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[6, 7], 2, &[6, 7, 10, 10]);
    test(&[10, 10, 10, 10], &[100, 101, 102], 10, &[98, 101, 102, 10]);
    test(&[10, 10, 10, 10], &[123, 456], 789, &[107, 456, 10, 10]);
    test(
        &[10, 10, 10, 10],
        &[0, 0, 456],
        789,
        &[0xffff_fceb, 0xffff_ffff, 455, 10],
    );
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 2 but the index is 2")]
fn limbs_neg_or_limb_to_out_fail_1() {
    limbs_neg_or_limb_to_out(&mut [10, 10], &[0, 0], 10);
}

#[test]
#[should_panic(expected = "assertion failed: out_limbs.len() >= len")]
fn limbs_neg_or_limb_to_out_fail_2() {
    limbs_neg_or_limb_to_out(&mut [10], &[10, 10], 10);
}

#[test]
fn test_limbs_neg_or_limb_in_place() {
    let test = |limbs: &[u32], limb: u32, out: &[u32]| {
        let mut limbs = limbs.to_vec();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        assert_eq!(limbs, out);
    };
    test(&[6, 7], 0, &[6, 7]);
    test(&[6, 7], 2, &[6, 7]);
    test(&[100, 101, 102], 10, &[98, 101, 102]);
    test(&[123, 456], 789, &[107, 456]);
    test(&[0, 0, 456], 789, &[0xffff_fceb, 0xffff_ffff, 455]);
}

#[test]
#[should_panic(expected = "index out of bounds: the len is 3 but the index is 3")]
fn limbs_neg_or_limb_in_place_fail() {
    limbs_neg_or_limb_in_place(&mut [0, 0, 0], 10);
}

#[test]
fn test_or_i32() {
    let test = |u, v: u32, out| {
        let mut n = Integer::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = rug::Integer::from_str(u).unwrap();
        n |= v;
        assert_eq!(n.to_string(), out);

        let n = Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);

        let n = &Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);

        let n = rug::Integer::from_str(u).unwrap() | v;
        assert_eq!(n.to_string(), out);

        let n = v | Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let n = v | &Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        assert_eq!(
            integer_or_u32_alt_1(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );
        assert_eq!(
            integer_or_u32_alt_2(&Integer::from_str(u).unwrap(), v).to_string(),
            out
        );

        let n = v | rug::Integer::from_str(u).unwrap();
        assert_eq!(n.to_string(), out);

        let mut n = rug::Integer::from(0);
        n.assign(v | &rug::Integer::from_str(u).unwrap());
        assert_eq!(n.to_string(), out);
    };

    test("0", 0, "0");
    test("0", 123, "123");
    test("123", 0, "123");
    test("123", 456, "507");
    test("999999999999", 123, "999999999999");
    test("1000000000000", 123, "1000000000123");
    test("1000000000001", 123, "1000000000123");
    test("12345678987654321", 0, "12345678987654321");
    test("12345678987654321", 456, "12345678987654649");
    test("12345678987654321", 987_654_321, "12345679395421361");

    test("-123", 0, "-123");
    test("-123", 456, "-51");
    test("-999999999999", 123, "-999999999877");
    test("-1000000000000", 123, "-999999999877");
    test("-1000000000001", 123, "-1000000000001");
    test("-12345678987654321", 0, "-12345678987654321");
    test("-12345678987654321", 456, "-12345678987654193");
    test("-12345678987654321", 987_654_321, "-12345678407767041");

    test("-4294967296", 0, "-4294967296");
    test("-4294967296", 3, "-4294967293");
    test("-293994983674745978880", 456, "-293994983674745978424");
    test(
        "-79228162514264337593543950336",
        456,
        "-79228162514264337593543949880",
    );
}

#[test]
fn limbs_neg_or_limb_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        assert_eq!(
            -Natural::from_owned_limbs_asc(limbs_neg_or_limb(limbs, limb)),
            -Natural::from_limbs_asc(limbs) | limb
        );
    });
}

#[test]
fn limbs_neg_or_limb_to_out_properties() {
    test_properties(
        triples_of_u32_vec_u32_vec_and_u32_var_3,
        |&(ref out_limbs, ref in_limbs, limb)| {
            let mut out_limbs = out_limbs.to_vec();
            let old_out_limbs = out_limbs.clone();
            limbs_neg_or_limb_to_out(&mut out_limbs, in_limbs, limb);
            let len = in_limbs.len();
            assert_eq!(
                -Natural::from_limbs_asc(&out_limbs[..len]),
                -Natural::from_limbs_asc(in_limbs) | limb,
            );
            assert_eq!(&out_limbs[len..], &old_out_limbs[len..]);
        },
    );
}

#[test]
fn limbs_neg_or_limb_in_place_properties() {
    test_properties(pairs_of_u32_vec_and_u32_var_1, |&(ref limbs, limb)| {
        let mut limbs = limbs.to_vec();
        let old_limbs = limbs.clone();
        limbs_neg_or_limb_in_place(&mut limbs, limb);
        assert_eq!(
            -Natural::from_limbs_asc(&limbs),
            -Natural::from_limbs_asc(&old_limbs) | limb
        );
    });
}

#[test]
fn or_u32_properties() {
    test_properties(
        pairs_of_integer_and_unsigned,
        |&(ref n, u): &(Integer, u32)| {
            let mut mut_n = n.clone();
            mut_n |= u;
            assert!(mut_n.is_valid());
            let result = mut_n;

            let mut rug_n = integer_to_rug_integer(n);
            rug_n |= u;
            assert_eq!(rug_integer_to_integer(&rug_n), result);

            let result_alt = n | u;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = n.clone() | u;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = u | n;
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            let result_alt = u | n.clone();
            assert!(result_alt.is_valid());
            assert_eq!(result_alt, result);

            assert_eq!(integer_or_u32_alt_1(&n, u), result);
            assert_eq!(integer_or_u32_alt_2(&n, u), result);

            assert_eq!(n | Integer::from(u), result);
            assert_eq!(Integer::from(u) | n, result);

            assert_eq!(&result | u, result);

            assert_eq!(
                rug_integer_to_integer(&(integer_to_rug_integer(n) | u)),
                result
            );
        },
    );

    test_properties(integers, |n| {
        assert_eq!(n | 0u32, *n);
        assert_eq!(0u32 | n, *n);
    });

    test_properties(unsigneds, |&u: &u32| {
        assert_eq!(&Integer::ZERO | u, u);
        assert_eq!(u | &Integer::ZERO, u);
        assert_eq!(&Integer::NEGATIVE_ONE | u, -1);
        assert_eq!(u | &Integer::NEGATIVE_ONE, -1);
    });
}