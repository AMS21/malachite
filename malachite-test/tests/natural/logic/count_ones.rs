use common::test_properties;
use malachite_base::num::IsPowerOfTwo;
use malachite_nz::natural::logic::count_ones::limbs_count_ones;
use malachite_nz::natural::Natural;
use malachite_test::inputs::base::vecs_of_unsigned;
use malachite_test::inputs::natural::naturals;
use std::str::FromStr;

#[test]
fn test_limbs_count_ones() {
    let test = |limbs, out| {
        assert_eq!(limbs_count_ones(limbs), out);
    };
    test(&[], 0);
    test(&[0, 1, 2], 2);
    test(&[1, 0xffff_ffff], 33);
}

#[test]
fn test_count_ones() {
    let test = |n, out| {
        assert_eq!(Natural::from_str(n).unwrap().count_ones(), out);
    };
    test("0", 0);
    test("105", 4);
    test("1000000000000", 13);
    test("4294967295", 32);
    test("4294967296", 1);
    test("18446744073709551615", 64);
    test("18446744073709551616", 1);
}

#[test]
fn limbs_count_ones_properties() {
    test_properties(vecs_of_unsigned, |limbs| {
        assert_eq!(
            limbs_count_ones(limbs),
            Natural::from_limbs_asc(limbs).count_ones()
        );
    });
}

#[test]
fn count_ones_properties() {
    test_properties(naturals, |x| {
        let ones = x.count_ones();
        assert_eq!(ones == 0, *x == 0);
        assert_eq!(ones == 1, x.is_power_of_two());
        assert_eq!((!x).checked_count_zeros(), Some(ones));
    });
}