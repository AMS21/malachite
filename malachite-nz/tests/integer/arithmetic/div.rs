use malachite_base::num::arithmetic::traits::DivRem;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::{NegativeOne, One, Zero};
use malachite_base::num::comparison::traits::PartialOrdAbs;
use malachite_base_test_util::generators::common::GenConfig;
use malachite_base_test_util::generators::signed_pair_gen_var_4;
use malachite_nz::integer::Integer;
use malachite_nz::platform::{Limb, SignedLimb};
use malachite_nz_test_util::common::{
    bigint_to_integer, integer_to_bigint, integer_to_rug_integer, rug_integer_to_integer,
};
use malachite_nz_test_util::generators::{
    integer_gen, integer_gen_var_8, integer_pair_gen_var_1, integer_pair_gen_var_2,
    natural_pair_gen_var_5,
};
use num::BigInt;
use std::str::FromStr;

#[test]
fn test_div() {
    let test = |s, t, quotient| {
        let u = Integer::from_str(s).unwrap();
        let v = Integer::from_str(t).unwrap();

        let mut x = u.clone();
        x /= v.clone();
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let mut x = u.clone();
        x /= &v;
        assert!(x.is_valid());
        assert_eq!(x.to_string(), quotient);

        let q = u.clone() / v.clone();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = u.clone() / &v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &u / v.clone();
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = &u / &v;
        assert!(q.is_valid());
        assert_eq!(q.to_string(), quotient);

        let q = BigInt::from_str(s).unwrap() / &BigInt::from_str(t).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = rug::Integer::from_str(s).unwrap() / rug::Integer::from_str(t).unwrap();
        assert_eq!(q.to_string(), quotient);

        let q = u.div_rem(v).0;
        assert_eq!(q.to_string(), quotient);
    };
    test("0", "1", "0");
    test("0", "123", "0");
    test("1", "1", "1");
    test("123", "1", "123");
    test("123", "123", "1");
    test("123", "456", "0");
    test("456", "123", "3");
    test("4294967295", "1", "4294967295");
    test("4294967295", "4294967295", "1");
    test("1000000000000", "1", "1000000000000");
    test("1000000000000", "3", "333333333333");
    test("1000000000000", "123", "8130081300");
    test("1000000000000", "4294967295", "232");
    test(
        "1000000000000000000000000",
        "1",
        "1000000000000000000000000",
    );
    test("1000000000000000000000000", "3", "333333333333333333333333");
    test("1000000000000000000000000", "123", "8130081300813008130081");
    test("1000000000000000000000000", "4294967295", "232830643708079");
    test("1000000000000000000000000", "1234567890987", "810000006723");
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
    );
    test(
        "253640751230376270397812803167",
        "2669936877441",
        "94998781946290113",
    );
    test("3768477692975601", "11447376614057827956", "0");
    test("3356605361737854", "3081095617839357", "1");
    test("1098730198198174614195", "953382298040157850476", "1");
    test(
        "69738658860594537152875081748",
        "69738658860594537152875081748",
        "1",
    );
    test(
        "1000000000000000000000000",
        "1000000000000000000000000",
        "1",
    );
    test("0", "1000000000000000000000000", "0");
    test("123", "1000000000000000000000000", "0");
    test(
        "915607705283450388306561139234228660872677067256472842161753852459689688332903348325308112\
        7923093090598913",
        "11669177832462215441614364516705357863717491965951",
        "784637716923245892498679555408392159158150581185689944063"
    );

    test("0", "-1", "0");
    test("0", "-123", "0");
    test("1", "-1", "-1");
    test("123", "-1", "-123");
    test("123", "-123", "-1");
    test("123", "-456", "0");
    test("456", "-123", "-3");
    test("4294967295", "-1", "-4294967295");
    test("4294967295", "-4294967295", "-1");
    test("1000000000000", "-1", "-1000000000000");
    test("1000000000000", "-3", "-333333333333");
    test("1000000000000", "-123", "-8130081300");
    test("1000000000000", "-4294967295", "-232");
    test(
        "1000000000000000000000000",
        "-1",
        "-1000000000000000000000000",
    );
    test(
        "1000000000000000000000000",
        "-3",
        "-333333333333333333333333",
    );
    test(
        "1000000000000000000000000",
        "-123",
        "-8130081300813008130081",
    );
    test(
        "1000000000000000000000000",
        "-4294967295",
        "-232830643708079",
    );
    test(
        "1000000000000000000000000",
        "-1234567890987",
        "-810000006723",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-1234567890987654321234567890987654321",
        "-810000006723000055638900467181273922269593923137018654",
    );
    test(
        "100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        0",
        "-316049380092839506236049380092839506176",
        "-3164062526261718967339454949926851258865601262253979",
    );
    test(
        "253640751230376270397812803167",
        "-2669936877441",
        "-94998781946290113",
    );
    test("3768477692975601", "-11447376614057827956", "0");
    test("3356605361737854", "-3081095617839357", "-1");
    test("1098730198198174614195", "-953382298040157850476", "-1");
    test(
        "69738658860594537152875081748",
        "-69738658860594537152875081748",
        "-1",
    );
    test(
        "1000000000000000000000000",
        "-1000000000000000000000000",
        "-1",
    );
    test("0", "-1000000000000000000000000", "0");
    test("123", "-1000000000000000000000000", "0");
    test(
        "915607705283450388306561139234228660872677067256472842161753852459689688332903348325308112\
        7923093090598913",
        "-11669177832462215441614364516705357863717491965951",
        "-784637716923245892498679555408392159158150581185689944063"
    );

    test("-1", "1", "-1");
    test("-123", "1", "-123");
    test("-123", "123", "-1");
    test("-123", "456", "0");
    test("-456", "123", "-3");
    test("-4294967295", "1", "-4294967295");
    test("-4294967295", "4294967295", "-1");
    test("-1000000000000", "1", "-1000000000000");
    test("-1000000000000", "3", "-333333333333");
    test("-1000000000000", "123", "-8130081300");
    test("-1000000000000", "4294967295", "-232");
    test(
        "-1000000000000000000000000",
        "1",
        "-1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "3",
        "-333333333333333333333333",
    );
    test(
        "-1000000000000000000000000",
        "123",
        "-8130081300813008130081",
    );
    test(
        "-1000000000000000000000000",
        "4294967295",
        "-232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "1234567890987",
        "-810000006723",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "1234567890987654321234567890987654321",
        "-810000006723000055638900467181273922269593923137018654",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "316049380092839506236049380092839506176",
        "-3164062526261718967339454949926851258865601262253979",
    );
    test(
        "-253640751230376270397812803167",
        "2669936877441",
        "-94998781946290113",
    );
    test("-3768477692975601", "11447376614057827956", "0");
    test("-3356605361737854", "3081095617839357", "-1");
    test("-1098730198198174614195", "953382298040157850476", "-1");
    test(
        "-69738658860594537152875081748",
        "69738658860594537152875081748",
        "-1",
    );
    test(
        "-1000000000000000000000000",
        "1000000000000000000000000",
        "-1",
    );
    test("-123", "1000000000000000000000000", "0");
    test(
        "-91560770528345038830656113923422866087267706725647284216175385245968968833290334832530811\
        27923093090598913",
        "11669177832462215441614364516705357863717491965951",
        "-784637716923245892498679555408392159158150581185689944063"
    );

    test("-1", "-1", "1");
    test("-123", "-1", "123");
    test("-123", "-123", "1");
    test("-123", "-456", "0");
    test("-456", "-123", "3");
    test("-4294967295", "-1", "4294967295");
    test("-4294967295", "-4294967295", "1");
    test("-1000000000000", "-1", "1000000000000");
    test("-1000000000000", "-3", "333333333333");
    test("-1000000000000", "-123", "8130081300");
    test("-1000000000000", "-4294967295", "232");
    test(
        "-1000000000000000000000000",
        "-1",
        "1000000000000000000000000",
    );
    test(
        "-1000000000000000000000000",
        "-3",
        "333333333333333333333333",
    );
    test(
        "-1000000000000000000000000",
        "-123",
        "8130081300813008130081",
    );
    test(
        "-1000000000000000000000000",
        "-4294967295",
        "232830643708079",
    );
    test(
        "-1000000000000000000000000",
        "-1234567890987",
        "810000006723",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-1234567890987654321234567890987654321",
        "810000006723000055638900467181273922269593923137018654",
    );
    test(
        "-10000000000000000000000000000000000000000000000000000000000000000000000000000000000000000\
        00",
        "-316049380092839506236049380092839506176",
        "3164062526261718967339454949926851258865601262253979",
    );
    test(
        "-253640751230376270397812803167",
        "-2669936877441",
        "94998781946290113",
    );
    test("-3768477692975601", "-11447376614057827956", "0");
    test("-3356605361737854", "-3081095617839357", "1");
    test("-1098730198198174614195", "-953382298040157850476", "1");
    test(
        "-69738658860594537152875081748",
        "-69738658860594537152875081748",
        "1",
    );
    test(
        "-1000000000000000000000000",
        "-1000000000000000000000000",
        "1",
    );
    test("-123", "-1000000000000000000000000", "0");
    test(
        "-91560770528345038830656113923422866087267706725647284216175385245968968833290334832530811\
        27923093090598913",
        "-11669177832462215441614364516705357863717491965951",
        "784637716923245892498679555408392159158150581185689944063"
    );
}

#[test]
#[should_panic]
fn div_assign_fail() {
    let mut x = Integer::from(10);
    x /= Integer::ZERO;
}

#[test]
#[should_panic]
fn div_assign_ref_fail() {
    let mut x = Integer::from(10);
    x /= &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn div_fail() {
    Integer::from(10) / Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn div_val_ref_fail() {
    Integer::from(10) / &Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn div_ref_val_fail() {
    &Integer::from(10) / Integer::ZERO;
}

#[test]
#[should_panic]
#[allow(unused_must_use, clippy::unnecessary_operation)]
fn div_ref_ref_fail() {
    &Integer::from(10) / &Integer::ZERO;
}

fn div_properties_helper(x: Integer, y: Integer) {
    let mut mut_x = x.clone();
    mut_x /= &y;
    assert!(mut_x.is_valid());
    let q = mut_x;

    let mut mut_x = x.clone();
    mut_x /= y.clone();
    let q_alt = mut_x;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = &x / &y;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = &x / y.clone();
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x.clone() / &y;
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = x.clone() / y.clone();
    assert!(q_alt.is_valid());
    assert_eq!(q_alt, q);

    let q_alt = (&x).div_rem(&y).0;
    assert_eq!(q_alt, q);

    let num_q = integer_to_bigint(&x) / &integer_to_bigint(&y);
    assert_eq!(bigint_to_integer(&num_q), q);

    let rug_q = integer_to_rug_integer(&x) / integer_to_rug_integer(&y);
    assert_eq!(rug_integer_to_integer(&rug_q), q);

    let remainder = &x - &q * &y;
    assert!(remainder.lt_abs(&y));
    assert!(remainder == 0 || (remainder > 0) == (x > 0));
    assert_eq!(&q * &y + remainder, x);
    assert_eq!((-&x) / &y, -&q);
    assert_eq!(x / (-y), -q);
}

#[allow(clippy::eq_op)]
#[test]
fn div_properties() {
    let mut config = GenConfig::new();
    config.insert("mean_bits_n", 2048);
    config.insert("mean_stripe_n", 16 << Limb::LOG_WIDTH);
    integer_pair_gen_var_1().test_properties_with_config(&config, |(x, y)| {
        div_properties_helper(x, y);
    });

    integer_pair_gen_var_2().test_properties_with_config(&config, |(x, y)| {
        div_properties_helper(x, y);
    });

    integer_gen().test_properties(|x| {
        assert_eq!(&x / Integer::ONE, x);
        assert_eq!(&x / Integer::NEGATIVE_ONE, -x);
    });

    integer_gen_var_8().test_properties(|x| {
        assert_eq!(Integer::ZERO / &x, 0);
        if x > Integer::ONE {
            assert_eq!(Integer::ONE / &x, 0);
        }
        assert_eq!(&x / Integer::ONE, x);
        assert_eq!(&x / Integer::NEGATIVE_ONE, -&x);
        assert_eq!(&x / &x, 1);
        assert_eq!(&x / -&x, -1);
    });

    natural_pair_gen_var_5().test_properties(|(x, y)| {
        assert_eq!(Integer::from(&x) / Integer::from(&y), x / y);
    });

    signed_pair_gen_var_4::<SignedLimb>().test_properties(|(x, y)| {
        assert_eq!(Integer::from(x) / Integer::from(y), x / y);
    });
}
