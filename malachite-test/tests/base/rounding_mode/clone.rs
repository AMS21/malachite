use common::LARGE_LIMIT;
use malachite_base::round::RoundingMode;
use malachite_test::common::GenerationMode;
use malachite_test::inputs::base::{pairs_of_rounding_modes, rounding_modes};

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn test_clone() {
    let test = |rm: RoundingMode| {
        let cloned = rm.clone();
        assert_eq!(cloned, rm);
    };
    test(RoundingMode::Down);
    test(RoundingMode::Up);
    test(RoundingMode::Floor);
    test(RoundingMode::Ceiling);
    test(RoundingMode::Nearest);
    test(RoundingMode::Exact);
}

#[test]
fn test_clone_from() {
    let test = |mut x: RoundingMode, y: RoundingMode| {
        x.clone_from(&y);
        assert_eq!(x, y);
    };
    test(RoundingMode::Exact, RoundingMode::Floor);
    test(RoundingMode::Up, RoundingMode::Ceiling);
}

#[test]
#[allow(unknown_lints, clone_on_copy)]
fn clone_and_clone_from_properties() {
    // x.clone() == x
    let one_rounding_mode = |rm: RoundingMode| {
        assert_eq!(rm.clone(), rm);
    };

    // x.clone_from(y); x == y
    let two_rounding_modes = |mut x: RoundingMode, y: RoundingMode| {
        x.clone_from(&y);
        assert_eq!(x, y);
    };

    for n in rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for n in rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        one_rounding_mode(n);
    }

    for (x, y) in pairs_of_rounding_modes(GenerationMode::Exhaustive).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }

    for (x, y) in pairs_of_rounding_modes(GenerationMode::Random(32)).take(LARGE_LIMIT) {
        two_rounding_modes(x, y);
    }
}
