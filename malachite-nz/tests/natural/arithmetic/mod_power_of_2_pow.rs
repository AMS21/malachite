use std::str::FromStr;

#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::arithmetic::traits::ModPowerOf2;
use malachite_base::num::arithmetic::traits::{
    ModPowerOf2IsReduced, ModPowerOf2Pow, ModPowerOf2PowAssign,
};
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::basic::integers::PrimitiveInt;
#[cfg(feature = "32_bit_limbs")]
use malachite_base::num::conversion::traits::ExactFrom;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz_test_util::natural::arithmetic::mod_power_of_2_pow::*;

#[cfg(feature = "32_bit_limbs")]
use malachite_nz::natural::arithmetic::mod_power_of_2_pow::{
    limbs_mod_power_of_2_pow, limbs_pow_low,
};
use malachite_nz::natural::Natural;
#[cfg(feature = "32_bit_limbs")]
use malachite_nz::platform::Limb;

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_pow_low(xs: &[Limb], es: &[Limb], out: &[Limb]) {
    let exp = Natural::from_limbs_asc(es);
    let n = xs.len();
    let pow = u64::exact_from(n) << Limb::LOG_WIDTH;
    let x = Natural::from_limbs_asc(xs).mod_power_of_2(pow);
    let expected = x.mod_power_of_2_pow(exp, pow);
    assert!(expected.mod_power_of_2_is_reduced(pow));
    assert_eq!(Natural::from_limbs_asc(&out[..n]), expected);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_pow_low() {
    let test = |xs: &[Limb], es: &[Limb], out: &[Limb]| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        let mut scratch = vec![0; xs.len()];
        limbs_pow_low(&mut xs, es, &mut scratch);
        assert_eq!(xs, out);
        verify_limbs_pow_low(xs_old, es, out);
    };
    // bit_index != 0 && !limbs_get_bit(es, bit_index - 1)
    // bit_index != 0 first time
    // bit_index >= window_size
    // this_windowsize == 1
    // bit_index != 0 second time
    // bit_index == 0 first time
    test(&[3], &[20], &[3486784401]);
    // bit_index < window_size
    // bit_index == 0 second time
    test(&[123, 456], &[789], &[426102667, 1687864191]);
    // this_windowsize > 1
    test(
        &[55455610, 1786865634],
        &[
            597666165, 1946668956, 2861877195, 1004122685, 3052222557, 4193145938, 1332420253,
            4049695026, 536465941, 13401346, 206750422, 2547236772, 718474167, 1253952310,
            4175135275, 3923178820, 3877868744,
        ],
        &[0, 0],
    );
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_low_fail_1() {
    let mut scratch = vec![];
    limbs_pow_low(&mut [], &[2], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_low_fail_2() {
    let mut scratch = vec![1];
    limbs_pow_low(&mut [1], &[], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_low_fail_3() {
    let mut scratch = vec![1];
    limbs_pow_low(&mut [1], &[2, 0], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_pow_low_fail_4() {
    let mut scratch = vec![1];
    limbs_pow_low(&mut [1], &[1], &mut scratch);
}

#[cfg(feature = "32_bit_limbs")]
fn verify_limbs_mod_power_of_2_pow(xs: &[Limb], es: &[Limb], pow: u64, out: &[Limb]) {
    let exp = Natural::from_limbs_asc(es);
    let x = Natural::from_limbs_asc(xs);
    assert!(x.mod_power_of_2_is_reduced(pow));
    let expected = (&x).mod_power_of_2_pow(&exp, pow);
    assert!(expected.mod_power_of_2_is_reduced(pow));
    assert_eq!(_simple_binary_mod_power_of_2_pow(&x, &exp, pow), expected);
    assert_eq!(Natural::from_limbs_asc(out), expected);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
fn test_limbs_mod_power_of_2_pow() {
    let test = |xs: &[Limb], es: &[Limb], pow: u64, out: &[Limb]| {
        let xs_old = xs;
        let mut xs = xs_old.to_vec();
        limbs_mod_power_of_2_pow(&mut xs, es, pow);
        assert_eq!(xs, out);
        verify_limbs_mod_power_of_2_pow(xs_old, es, pow, out);
    };
    test(&[1], &[2], 2, &[1]);
    test(&[3], &[2], 2, &[1]);
    test(&[3], &[3], 4, &[11]);
    test(&[25], &[10, 10], 5, &[17]);
    test(&[123, 456], &[789, 987], 42, &[426102667, 987]);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_pow_fail_1() {
    limbs_mod_power_of_2_pow(&mut vec![2], &[2, 0], 5);
}

#[cfg(feature = "32_bit_limbs")]
#[test]
#[should_panic]
fn limbs_mod_power_of_2_pow_fail_2() {
    limbs_mod_power_of_2_pow(&mut vec![2], &[1], 5);
}

#[test]
fn test_mod_power_of_2_pow() {
    let test = |s, t, pow, out| {
        let u = Natural::from_str(s).unwrap();
        let exp = Natural::from_str(t).unwrap();
        assert!(u.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_pow_assign(exp.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
        assert!(n.mod_power_of_2_is_reduced(pow));

        let mut n = u.clone();
        n.mod_power_of_2_pow_assign(&exp, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_pow(exp.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_pow(exp.clone(), pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = u.clone().mod_power_of_2_pow(&exp, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let n = (&u).mod_power_of_2_pow(&exp, pow);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", "0", 0, "0");
    test("0", "0", 10, "1");
    test("0", "1", 10, "0");
    test("2", "10", 8, "0");
    test("3", "10", 8, "169");
    test("10", "1000", 30, "0");
    test("11", "1000", 30, "289109473");
    test("3", "1000000", 100, "1176684907284103408190379631873");
    test(
        "123456789",
        "1000000000",
        100,
        "1180978940853570377595087681537",
    );
}
