use std::str::FromStr;

use malachite_base::conversion::{CheckedFrom, RoundingFrom};
use malachite_base::crement::Crementable;
use malachite_base::num::floats::PrimitiveFloat;
use malachite_base::num::traits::{NegativeOne, One, Parity, Two, Zero};
use malachite_base::round::RoundingMode;
use malachite_nz::natural::Natural;

use common::test_properties;
use malachite_test::inputs::natural::{
    naturals, naturals_exactly_equal_to_f32, naturals_exactly_equal_to_f64,
    naturals_not_exactly_equal_to_f32, naturals_not_exactly_equal_to_f64, naturals_var_2_f32,
    naturals_var_2_f64, pairs_of_natural_and_rounding_mode_var_1_f32,
    pairs_of_natural_and_rounding_mode_var_1_f64,
};

//TODO move
#[test]
fn test_f32() {
    let test = |f: f32, out| {
        assert_eq!(f.to_bits(), out);
    };
    test(f32::NAN, 0x7fc0_0000);
    test(f32::ZERO, 0);
    test(f32::NEGATIVE_ZERO, 0x8000_0000);
    test(f32::MIN_POSITIVE, 1);
    test(f32::MAX_SUBNORMAL, 0x7f_ffff);
    test(f32::MIN_POSITIVE_NORMAL, 0x80_0000);
    test(f32::ONE, 0x3f80_0000);
    test(f32::NEGATIVE_ONE, 0xbf80_0000);
    test(f32::TWO, 0x4000_0000);
    test(f32::MAX_FINITE, 0x7f7f_ffff);
    test(f32::MIN_FINITE, 0xff7f_ffff);
    test(f32::POSITIVE_INFINITY, 0x7f80_0000);
    test(f32::NEGATIVE_INFINITY, 0xff80_0000);

    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 0x100_0001);
    assert_eq!(f32::SMALLEST_UNREPRESENTABLE_UINT, 16_777_217);
}

#[test]
fn test_f64() {
    let test = |f: f64, out| {
        assert_eq!(f.to_bits(), out);
    };
    test(f64::NAN, 0x7ff8_0000_0000_0000);
    test(f64::ZERO, 0);
    test(f64::NEGATIVE_ZERO, 0x8000_0000_0000_0000);
    test(f64::MIN_POSITIVE, 1);
    test(f64::MAX_SUBNORMAL, 0xf_ffff_ffff_ffff);
    test(f64::MIN_POSITIVE_NORMAL, 0x10_0000_0000_0000);
    test(f64::ONE, 0x3ff0_0000_0000_0000);
    test(f64::NEGATIVE_ONE, 0xbff0_0000_0000_0000);
    test(f64::TWO, 0x4000_0000_0000_0000);
    test(f64::MAX_FINITE, 0x7fef_ffff_ffff_ffff);
    test(f64::MIN_FINITE, 0xffef_ffff_ffff_ffff);
    test(f64::POSITIVE_INFINITY, 0x7ff0_0000_0000_0000);
    test(f64::NEGATIVE_INFINITY, 0xfff0_0000_0000_0000);

    assert_eq!(f64::SMALLEST_UNREPRESENTABLE_UINT, 0x20_0000_0000_0001);
    assert_eq!(f64::SMALLEST_UNREPRESENTABLE_UINT, 9_007_199_254_740_993);
}

#[test]
fn test_f32_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out| {
        assert_eq!(f32::rounding_from(Natural::from_str(n).unwrap(), rm), out);
    };
    test("3", RoundingMode::Exact, 3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("1000000000", RoundingMode::Exact, 1.0e9);
    test("16777216", RoundingMode::Exact, 1.6777216e7);
    test("16777218", RoundingMode::Exact, 1.6777218e7);
    test("16777217", RoundingMode::Floor, 1.6777216e7);
    test("16777217", RoundingMode::Down, 1.6777216e7);
    test("16777217", RoundingMode::Ceiling, 1.6777218e7);
    test("16777217", RoundingMode::Up, 1.6777218e7);
    test("16777217", RoundingMode::Nearest, 1.6777216e7);
    test("33554432", RoundingMode::Exact, 3.3554432e7);
    test("33554436", RoundingMode::Exact, 3.3554436e7);
    test("33554433", RoundingMode::Floor, 3.3554432e7);
    test("33554433", RoundingMode::Down, 3.3554432e7);
    test("33554433", RoundingMode::Ceiling, 3.3554436e7);
    test("33554433", RoundingMode::Up, 3.3554436e7);
    test("33554433", RoundingMode::Nearest, 3.3554432e7);
    test("33554434", RoundingMode::Nearest, 3.3554432e7);
    test("33554435", RoundingMode::Nearest, 3.3554436e7);
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Floor,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Down,
        3.4028233e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Ceiling,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Up,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925439",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925440",
        RoundingMode::Exact,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "340282346638528859811704183484516925441",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Floor,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Down,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Nearest,
        3.4028235e38,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Ceiling,
        f32::POSITIVE_INFINITY,
    );
    test(
        "10000000000000000000000000000000000000000000000000000",
        RoundingMode::Up,
        f32::POSITIVE_INFINITY,
    );
    test("1125899873419263", RoundingMode::Floor, 1.12589984e15);
    test("1125899873419263", RoundingMode::Down, 1.12589984e15);
    test("1125899873419263", RoundingMode::Ceiling, 1.1258999e15);
    test("1125899873419263", RoundingMode::Up, 1.1258999e15);
    test("1125899873419263", RoundingMode::Nearest, 1.1258999e15);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_1() {
    f32::rounding_from(
        Natural::from_str("340282346638528859811704183484516925439").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_2() {
    f32::rounding_from(
        Natural::from_str("340282346638528859811704183484516925441").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_3() {
    f32::rounding_from(Natural::from_str("16777217").unwrap(), RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f32_rounding_from_natural_fail_4() {
    f32::rounding_from(
        Natural::from_str("10000000000000000000000000000000000000000000000000000").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
fn test_f64_rounding_from_natural() {
    let test = |n: &str, rm: RoundingMode, out| {
        assert_eq!(f64::rounding_from(Natural::from_str(n).unwrap(), rm), out);
    };
    test("3", RoundingMode::Exact, 3.0);
    test("123", RoundingMode::Exact, 123.0);
    test("0", RoundingMode::Exact, 0.0);
    test("100000000000000000000", RoundingMode::Exact, 1.0e20);
    test(
        "9007199254740992",
        RoundingMode::Exact,
        9.007199254740992e15,
    );
    test(
        "9007199254740994",
        RoundingMode::Exact,
        9.007199254740994e15,
    );
    test(
        "9007199254740993",
        RoundingMode::Floor,
        9.007199254740992e15,
    );
    test("9007199254740993", RoundingMode::Down, 9.007199254740992e15);
    test(
        "9007199254740993",
        RoundingMode::Ceiling,
        9.007199254740994e15,
    );
    test("9007199254740993", RoundingMode::Up, 9.007199254740994e15);
    test(
        "9007199254740993",
        RoundingMode::Nearest,
        9.007199254740992e15,
    );
    test(
        "18014398509481984",
        RoundingMode::Exact,
        1.8014398509481984e16,
    );
    test(
        "18014398509481988",
        RoundingMode::Exact,
        1.8014398509481988e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Floor,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Down,
        1.8014398509481984e16,
    );
    test(
        "18014398509481985",
        RoundingMode::Ceiling,
        1.8014398509481988e16,
    );
    test("18014398509481985", RoundingMode::Up, 1.8014398509481988e16);
    test(
        "18014398509481985",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "18014398509481986",
        RoundingMode::Nearest,
        1.8014398509481984e16,
    );
    test(
        "18014398509481987",
        RoundingMode::Nearest,
        1.8014398509481988e16,
    );
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Floor, 1.7976931348623155e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Down, 1.7976931348623155e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Ceiling, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Up, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", RoundingMode::Nearest, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", RoundingMode::Exact, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Floor, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Down, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Nearest, 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Ceiling, f64::POSITIVE_INFINITY);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", RoundingMode::Up, f64::POSITIVE_INFINITY);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_1() {
    f64::rounding_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367").unwrap(),
    RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_2() {
    f64::rounding_from(Natural::from_str(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369").unwrap(),
    RoundingMode::Exact);
}

#[test]
#[should_panic]
fn f64_rounding_from_natural_fail_3() {
    f64::rounding_from(
        Natural::from_str("9007199254740993").unwrap(),
        RoundingMode::Exact,
    );
}

#[test]
fn test_f32_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::from(Natural::from_str(n).unwrap()), out);
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("16777216", 1.6777216e7);
    test("16777218", 1.6777218e7);
    test("16777217", 1.6777216e7);
    test("33554432", 3.3554432e7);
    test("33554436", 3.3554436e7);
    test("33554433", 3.3554432e7);
    test("33554434", 3.3554432e7);
    test("33554435", 3.3554436e7);
    test("340282346638528859811704183484516925439", 3.4028235e38);
    test("340282346638528859811704183484516925440", 3.4028235e38);
    test("340282346638528859811704183484516925441", 3.4028235e38);
    test(
        "10000000000000000000000000000000000000000000000000000",
        3.4028235e38,
    );
}

#[test]
fn test_f64_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::from(Natural::from_str(n).unwrap()), out);
    };
    test("3", 3.0);
    test("123", 123.0);
    test("0", 0.0);
    test("1000000000", 1.0e9);
    test("9007199254740992", 9.007199254740992e15);
    test("9007199254740994", 9.007199254740994e15);
    test("9007199254740993", 9.007199254740992e15);
    test("18014398509481984", 1.8014398509481984e16);
    test("18014398509481988", 1.8014398509481988e16);
    test("18014398509481985", 1.8014398509481984e16);
    test("18014398509481986", 1.8014398509481984e16);
    test("18014398509481987", 1.8014398509481988e16);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", 1.7976931348623157e308);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", 1.7976931348623157e308);
}

#[test]
fn test_f32_checked_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f32::checked_from(Natural::from_str(n).unwrap()), out);
    };
    test("3", Some(3.0));
    test("123", Some(123.0));
    test("0", Some(0.0));
    test("1000000000", Some(1.0e9));
    test("16777216", Some(1.6777216e7));
    test("16777218", Some(1.6777218e7));
    test("16777217", None);
    test("33554432", Some(3.3554432e7));
    test("33554436", Some(3.3554436e7));
    test("33554433", None);
    test("33554434", None);
    test("33554435", None);
    test("340282346638528859811704183484516925439", None);
    test(
        "340282346638528859811704183484516925440",
        Some(3.4028235e38),
    );
    test("340282346638528859811704183484516925441", None);
    test(
        "10000000000000000000000000000000000000000000000000000",
        None,
    );
}

#[test]
fn test_f64_checked_from_natural() {
    let test = |n: &str, out| {
        assert_eq!(f64::checked_from(Natural::from_str(n).unwrap()), out);
    };
    test("3", Some(3.0));
    test("123", Some(123.0));
    test("0", Some(0.0));
    test("1000000000", Some(1.0e9));
    test("9007199254740992", Some(9.007199254740992e15));
    test("9007199254740994", Some(9.007199254740994e15));
    test("9007199254740993", None);
    test("18014398509481984", Some(1.8014398509481984e16));
    test("18014398509481988", Some(1.8014398509481988e16));
    test("18014398509481985", None);
    test("18014398509481986", None);
    test("18014398509481987", None);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858367", None);
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858368", Some(1.7976931348623157e308));
    test(
        "179769313486231570814527423731704356798070567525844996598917476803157260780028538760589558\
        6327668781715404589535143824642343213268894641827684675467035375169860499105765512820762454\
        9009038932894407586850845513394230458323690322294816580855933212334827479782620414472316873\
        8177180919299881250404026184124858369", None);
}

macro_rules! float_properties {
    (
        $f: ident,
        $pairs_of_natural_and_rounding_mode_var_1: ident,
        $naturals_exactly_equal_to_float: ident,
        $naturals_not_exactly_equal_to_float: ident,
        $naturals_var_2: ident,
        $float_rounding_from_natural_properties: ident,
        $float_from_natural_properties: ident,
        $float_checked_from_natural_properties: ident
    ) => {
        #[test]
        fn $float_rounding_from_natural_properties() {
            test_properties($pairs_of_natural_and_rounding_mode_var_1, |&(ref n, rm)| {
                let f = $f::rounding_from(n, rm);
                assert_eq!($f::rounding_from(n.clone(), rm), f);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::rounding_from(n, RoundingMode::Exact);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Exact), f);
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Floor));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Ceiling));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Down));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Up));
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Nearest));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Floor));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Ceiling));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Down));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Up));
                assert_eq!(f, $f::rounding_from(n.clone(), RoundingMode::Nearest));
                assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let mut f_above = f_below;
                f_above.increment();
                assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Ceiling));
                assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Ceiling));
                assert_eq!(f_below, $f::rounding_from(n, RoundingMode::Down));
                assert_eq!(f_below, $f::rounding_from(n.clone(), RoundingMode::Down));
                assert_eq!(f_above, $f::rounding_from(n, RoundingMode::Up));
                assert_eq!(f_above, $f::rounding_from(n.clone(), RoundingMode::Up));
                let f_nearest = $f::rounding_from(n, RoundingMode::Nearest);
                assert_eq!(
                    $f::rounding_from(n.clone(), RoundingMode::Nearest),
                    f_nearest
                );
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Natural::from(f_nearest), *n);
            });

            test_properties($naturals_var_2, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let mut ceiling = floor;
                ceiling.increment();
                let nearest = $f::rounding_from(n, RoundingMode::Nearest);
                assert_eq!(
                    nearest,
                    if floor.to_bits().even() {
                        floor
                    } else {
                        ceiling
                    }
                );
            });
        }

        #[test]
        fn $float_from_natural_properties() {
            test_properties(naturals, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!($f::rounding_from(n, RoundingMode::Nearest), f);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::from(n);
                assert_eq!($f::from(n.clone()), f);
                assert_eq!(Natural::from(f), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                let f_below = $f::rounding_from(n, RoundingMode::Floor);
                assert_eq!($f::rounding_from(n.clone(), RoundingMode::Floor), f_below);
                let mut f_above = f_below;
                f_above.increment();
                let f_nearest = $f::from(n);
                assert_eq!($f::from(n.clone()), f_nearest);
                assert!(f_nearest == f_below || f_nearest == f_above);
                assert_ne!(Natural::from(f_nearest), *n);
            });

            test_properties($naturals_var_2, |n| {
                let floor = $f::rounding_from(n, RoundingMode::Floor);
                let mut ceiling = floor;
                ceiling.increment();
                let nearest = $f::from(n);
                assert_eq!(
                    nearest,
                    if floor.to_bits().even() {
                        floor
                    } else {
                        ceiling
                    }
                );
            });
        }

        #[test]
        fn $float_checked_from_natural_properties() {
            test_properties(naturals, |n| {
                let of = $f::checked_from(n);
                assert_eq!($f::checked_from(n.clone()), of);
            });

            test_properties($naturals_exactly_equal_to_float, |n| {
                let f = $f::checked_from(n).unwrap();
                assert_eq!($f::checked_from(n.clone()).unwrap(), f);
                assert_eq!(f, $f::rounding_from(n, RoundingMode::Exact));
                assert_eq!(Natural::rounding_from(f, RoundingMode::Exact), *n);
            });

            test_properties($naturals_not_exactly_equal_to_float, |n| {
                assert!($f::checked_from(n).is_none());
            });

            test_properties($naturals_var_2, |n| {
                assert!($f::checked_from(n).is_none());
            });
        }
    };
}

float_properties!(
    f32,
    pairs_of_natural_and_rounding_mode_var_1_f32,
    naturals_exactly_equal_to_f32,
    naturals_not_exactly_equal_to_f32,
    naturals_var_2_f32,
    f32_rounding_from_natural_properties,
    f32_from_natural_properties,
    f32_checked_from_natural_properties
);
float_properties!(
    f64,
    pairs_of_natural_and_rounding_mode_var_1_f64,
    naturals_exactly_equal_to_f64,
    naturals_not_exactly_equal_to_f64,
    naturals_var_2_f64,
    f64_rounding_from_natural_properties,
    f64_from_natural_properties,
    f64_checked_from_natural_properties
);