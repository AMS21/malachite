use malachite_base::num::basic::floats::PrimitiveFloat;
use malachite_base::num::exhaustive::primitive_float_increasing_inclusive_range;
use malachite_base::test_util::num::exhaustive::*;
use std::panic::catch_unwind;

fn primitive_float_increasing_inclusive_range_helper<T: PrimitiveFloat>(
    a: T,
    b: T,
    first_20: &[T],
    last_20: &[T],
) {
    exhaustive_primitive_floats_helper_helper_with_reverse(
        primitive_float_increasing_inclusive_range::<T>(a, b),
        first_20,
        last_20,
    );
}

#[allow(clippy::approx_constant)]
#[test]
fn test_primitive_float_increasing_inclusive_range() {
    primitive_float_increasing_inclusive_range_helper::<f32>(1.0, 1.0, &[1.0], &[1.0]);
    primitive_float_increasing_inclusive_range_helper::<f32>(
        1.0,
        2.0,
        &[
            1.0, 1.0000001, 1.0000002, 1.0000004, 1.0000005, 1.0000006, 1.0000007, 1.0000008,
            1.000001, 1.0000011, 1.0000012, 1.0000013, 1.0000014, 1.0000015, 1.0000017, 1.0000018,
            1.0000019, 1.000002, 1.0000021, 1.0000023,
        ],
        &[
            1.9999977, 1.9999979, 1.999998, 1.9999981, 1.9999982, 1.9999983, 1.9999985, 1.9999986,
            1.9999987, 1.9999988, 1.9999989, 1.999999, 1.9999992, 1.9999993, 1.9999994, 1.9999995,
            1.9999996, 1.9999998, 1.9999999, 2.0,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        -0.1,
        0.1,
        &[
            -0.1,
            -0.099999994,
            -0.09999999,
            -0.09999998,
            -0.09999997,
            -0.099999964,
            -0.09999996,
            -0.09999995,
            -0.09999994,
            -0.099999934,
            -0.09999993,
            -0.09999992,
            -0.09999991,
            -0.099999905,
            -0.0999999,
            -0.09999989,
            -0.09999988,
            -0.099999875,
            -0.09999987,
            -0.09999986,
        ],
        &[
            0.09999986,
            0.09999987,
            0.099999875,
            0.09999988,
            0.09999989,
            0.0999999,
            0.099999905,
            0.09999991,
            0.09999992,
            0.09999993,
            0.099999934,
            0.09999994,
            0.09999995,
            0.09999996,
            0.099999964,
            0.09999997,
            0.09999998,
            0.09999999,
            0.099999994,
            0.1,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        core::f32::consts::E,
        core::f32::consts::PI,
        &[
            core::f32::consts::E,
            2.718282,
            2.7182822,
            2.7182825,
            2.7182827,
            2.718283,
            2.7182832,
            2.7182834,
            2.7182837,
            2.718284,
            2.7182841,
            2.7182844,
            2.7182846,
            2.7182848,
            2.718285,
            2.7182853,
            2.7182856,
            2.7182858,
            2.718286,
            2.7182863,
        ],
        &[
            3.1415882,
            3.1415884,
            3.1415887,
            3.141589,
            3.1415892,
            3.1415894,
            3.1415896,
            3.1415899,
            3.14159,
            3.1415904,
            3.1415906,
            3.1415908,
            3.141591,
            3.1415913,
            3.1415915,
            3.1415918,
            3.141592,
            3.1415923,
            3.1415925,
            core::f32::consts::PI,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        100.0,
        101.0,
        &[
            100.0, 100.00001, 100.000015, 100.00002, 100.00003, 100.00004, 100.000046, 100.00005,
            100.00006, 100.00007, 100.00008, 100.000084, 100.00009, 100.0001, 100.00011,
            100.000114, 100.00012, 100.00013, 100.00014, 100.000145,
        ],
        &[
            100.999855, 100.99986, 100.99987, 100.99988, 100.999886, 100.99989, 100.9999,
            100.99991, 100.999916, 100.99992, 100.99993, 100.99994, 100.99995, 100.999954,
            100.99996, 100.99997, 100.99998, 100.999985, 100.99999, 101.0,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        1.0e38,
        f32::POSITIVE_INFINITY,
        &[
            1.0e38,
            1.0000001e38,
            1.0000002e38,
            1.0000003e38,
            1.0000004e38,
            1.0000005e38,
            1.0000006e38,
            1.0000007e38,
            1.0000008e38,
            1.0000009e38,
            1.000001e38,
            1.0000011e38,
            1.0000012e38,
            1.0000013e38,
            1.0000014e38,
            1.0000015e38,
            1.0000016e38,
            1.0000017e38,
            1.0000018e38,
            1.0000019e38,
        ],
        &[
            3.4028198e38,
            3.40282e38,
            3.4028202e38,
            3.4028204e38,
            3.4028206e38,
            3.4028208e38,
            3.402821e38,
            3.4028212e38,
            3.4028214e38,
            3.4028216e38,
            3.4028218e38,
            3.402822e38,
            3.4028222e38,
            3.4028225e38,
            3.4028227e38,
            3.4028229e38,
            3.402823e38,
            3.4028233e38,
            3.4028235e38,
            f32::POSITIVE_INFINITY,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[-1.0e-45, -0.0, 0.0, 1.0e-45],
        &[-1.0e-45, -0.0, 0.0, 1.0e-45],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        -0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[-0.0, 0.0, 1.0e-45],
        &[-0.0, 0.0, 1.0e-45],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        0.0,
        f32::MIN_POSITIVE_SUBNORMAL,
        &[0.0, 1.0e-45],
        &[0.0, 1.0e-45],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        &[-1.0e-45, -0.0],
        &[-1.0e-45, -0.0],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        -f32::MIN_POSITIVE_SUBNORMAL,
        0.0,
        &[-1.0e-45, -0.0, 0.0],
        &[-1.0e-45, -0.0, 0.0],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        &[
            f32::NEGATIVE_INFINITY,
            -3.4028235e38,
            -3.4028233e38,
            -3.402823e38,
            -3.4028229e38,
            -3.4028227e38,
            -3.4028225e38,
            -3.4028222e38,
            -3.402822e38,
            -3.4028218e38,
            -3.4028216e38,
            -3.4028214e38,
            -3.4028212e38,
            -3.402821e38,
            -3.4028208e38,
            -3.4028206e38,
            -3.4028204e38,
            -3.4028202e38,
            -3.40282e38,
            -3.4028198e38,
        ],
        &[
            3.4028198e38,
            3.40282e38,
            3.4028202e38,
            3.4028204e38,
            3.4028206e38,
            3.4028208e38,
            3.402821e38,
            3.4028212e38,
            3.4028214e38,
            3.4028216e38,
            3.4028218e38,
            3.402822e38,
            3.4028222e38,
            3.4028225e38,
            3.4028227e38,
            3.4028229e38,
            3.402823e38,
            3.4028233e38,
            3.4028235e38,
            f32::POSITIVE_INFINITY,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        f32::NEGATIVE_INFINITY,
        f32::NEGATIVE_INFINITY,
        &[f32::NEGATIVE_INFINITY],
        &[f32::NEGATIVE_INFINITY],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(
        f32::POSITIVE_INFINITY,
        f32::POSITIVE_INFINITY,
        &[f32::POSITIVE_INFINITY],
        &[f32::POSITIVE_INFINITY],
    );
    primitive_float_increasing_inclusive_range_helper::<f32>(0.0, 0.0, &[0.0], &[0.0]);
    primitive_float_increasing_inclusive_range_helper::<f32>(-0.0, -0.0, &[-0.0], &[-0.0]);
    primitive_float_increasing_inclusive_range_helper::<f32>(-0.0, 0.0, &[-0.0, 0.0], &[-0.0, 0.0]);

    primitive_float_increasing_inclusive_range_helper::<f64>(1.0, 1.0, &[1.0], &[1.0]);
    primitive_float_increasing_inclusive_range_helper::<f64>(
        1.0,
        2.0,
        &[
            1.0,
            1.0000000000000002,
            1.0000000000000004,
            1.0000000000000007,
            1.0000000000000009,
            1.000000000000001,
            1.0000000000000013,
            1.0000000000000016,
            1.0000000000000018,
            1.000000000000002,
            1.0000000000000022,
            1.0000000000000024,
            1.0000000000000027,
            1.0000000000000029,
            1.000000000000003,
            1.0000000000000033,
            1.0000000000000036,
            1.0000000000000038,
            1.000000000000004,
            1.0000000000000042,
        ],
        &[
            1.9999999999999958,
            1.999999999999996,
            1.9999999999999962,
            1.9999999999999964,
            1.9999999999999967,
            1.999999999999997,
            1.9999999999999971,
            1.9999999999999973,
            1.9999999999999976,
            1.9999999999999978,
            1.999999999999998,
            1.9999999999999982,
            1.9999999999999984,
            1.9999999999999987,
            1.999999999999999,
            1.9999999999999991,
            1.9999999999999993,
            1.9999999999999996,
            1.9999999999999998,
            2.0,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        -0.1,
        0.1,
        &[
            -0.1,
            -0.09999999999999999,
            -0.09999999999999998,
            -0.09999999999999996,
            -0.09999999999999995,
            -0.09999999999999994,
            -0.09999999999999992,
            -0.09999999999999991,
            -0.0999999999999999,
            -0.09999999999999988,
            -0.09999999999999987,
            -0.09999999999999985,
            -0.09999999999999984,
            -0.09999999999999983,
            -0.09999999999999981,
            -0.0999999999999998,
            -0.09999999999999978,
            -0.09999999999999977,
            -0.09999999999999976,
            -0.09999999999999974,
        ],
        &[
            0.09999999999999974,
            0.09999999999999976,
            0.09999999999999977,
            0.09999999999999978,
            0.0999999999999998,
            0.09999999999999981,
            0.09999999999999983,
            0.09999999999999984,
            0.09999999999999985,
            0.09999999999999987,
            0.09999999999999988,
            0.0999999999999999,
            0.09999999999999991,
            0.09999999999999992,
            0.09999999999999994,
            0.09999999999999995,
            0.09999999999999996,
            0.09999999999999998,
            0.09999999999999999,
            0.1,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        core::f64::consts::E,
        core::f64::consts::PI,
        &[
            core::f64::consts::E,
            2.7182818284590455,
            2.718281828459046,
            2.7182818284590464,
            2.718281828459047,
            2.7182818284590473,
            2.7182818284590478,
            2.718281828459048,
            2.7182818284590486,
            2.718281828459049,
            2.7182818284590495,
            2.71828182845905,
            2.7182818284590504,
            2.718281828459051,
            2.7182818284590513,
            2.7182818284590518,
            2.718281828459052,
            2.7182818284590526,
            2.718281828459053,
            2.7182818284590535,
        ],
        &[
            3.1415926535897847,
            3.141592653589785,
            3.1415926535897856,
            3.141592653589786,
            3.1415926535897865,
            3.141592653589787,
            3.1415926535897873,
            3.141592653589788,
            3.1415926535897882,
            3.1415926535897887,
            3.141592653589789,
            3.1415926535897896,
            3.14159265358979,
            3.1415926535897905,
            3.141592653589791,
            3.1415926535897913,
            3.141592653589792,
            3.1415926535897922,
            3.1415926535897927,
            core::f64::consts::PI,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        100.0,
        101.0,
        &[
            100.0,
            100.00000000000001,
            100.00000000000003,
            100.00000000000004,
            100.00000000000006,
            100.00000000000007,
            100.00000000000009,
            100.0000000000001,
            100.00000000000011,
            100.00000000000013,
            100.00000000000014,
            100.00000000000016,
            100.00000000000017,
            100.00000000000018,
            100.0000000000002,
            100.00000000000021,
            100.00000000000023,
            100.00000000000024,
            100.00000000000026,
            100.00000000000027,
        ],
        &[
            100.99999999999973,
            100.99999999999974,
            100.99999999999976,
            100.99999999999977,
            100.99999999999979,
            100.9999999999998,
            100.99999999999982,
            100.99999999999983,
            100.99999999999984,
            100.99999999999986,
            100.99999999999987,
            100.99999999999989,
            100.9999999999999,
            100.99999999999991,
            100.99999999999993,
            100.99999999999994,
            100.99999999999996,
            100.99999999999997,
            100.99999999999999,
            101.0,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        1.0e308,
        f64::POSITIVE_INFINITY,
        &[
            1.0e308,
            1.0000000000000002e308,
            1.0000000000000004e308,
            1.0000000000000006e308,
            1.0000000000000008e308,
            1.000000000000001e308,
            1.0000000000000012e308,
            1.0000000000000014e308,
            1.0000000000000016e308,
            1.0000000000000018e308,
            1.000000000000002e308,
            1.0000000000000022e308,
            1.0000000000000024e308,
            1.0000000000000026e308,
            1.0000000000000028e308,
            1.000000000000003e308,
            1.0000000000000032e308,
            1.0000000000000034e308,
            1.0000000000000036e308,
            1.0000000000000038e308,
        ],
        &[
            1.7976931348623121e308,
            1.7976931348623123e308,
            1.7976931348623125e308,
            1.7976931348623127e308,
            1.797693134862313e308,
            1.7976931348623131e308,
            1.7976931348623133e308,
            1.7976931348623135e308,
            1.7976931348623137e308,
            1.797693134862314e308,
            1.7976931348623141e308,
            1.7976931348623143e308,
            1.7976931348623145e308,
            1.7976931348623147e308,
            1.797693134862315e308,
            1.7976931348623151e308,
            1.7976931348623153e308,
            1.7976931348623155e308,
            1.7976931348623157e308,
            f64::POSITIVE_INFINITY,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[-5.0e-324, -0.0, 0.0, 5.0e-324],
        &[-5.0e-324, -0.0, 0.0, 5.0e-324],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        -0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[-0.0, 0.0, 5.0e-324],
        &[-0.0, 0.0, 5.0e-324],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        0.0,
        f64::MIN_POSITIVE_SUBNORMAL,
        &[0.0, 5.0e-324],
        &[0.0, 5.0e-324],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        -0.0,
        &[-5.0e-324, -0.0],
        &[-5.0e-324, -0.0],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        -f64::MIN_POSITIVE_SUBNORMAL,
        0.0,
        &[-5.0e-324, -0.0, 0.0],
        &[-5.0e-324, -0.0, 0.0],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        &[
            f64::NEGATIVE_INFINITY,
            -1.7976931348623157e308,
            -1.7976931348623155e308,
            -1.7976931348623153e308,
            -1.7976931348623151e308,
            -1.797693134862315e308,
            -1.7976931348623147e308,
            -1.7976931348623145e308,
            -1.7976931348623143e308,
            -1.7976931348623141e308,
            -1.797693134862314e308,
            -1.7976931348623137e308,
            -1.7976931348623135e308,
            -1.7976931348623133e308,
            -1.7976931348623131e308,
            -1.797693134862313e308,
            -1.7976931348623127e308,
            -1.7976931348623125e308,
            -1.7976931348623123e308,
            -1.7976931348623121e308,
        ],
        &[
            1.7976931348623121e308,
            1.7976931348623123e308,
            1.7976931348623125e308,
            1.7976931348623127e308,
            1.797693134862313e308,
            1.7976931348623131e308,
            1.7976931348623133e308,
            1.7976931348623135e308,
            1.7976931348623137e308,
            1.797693134862314e308,
            1.7976931348623141e308,
            1.7976931348623143e308,
            1.7976931348623145e308,
            1.7976931348623147e308,
            1.797693134862315e308,
            1.7976931348623151e308,
            1.7976931348623153e308,
            1.7976931348623155e308,
            1.7976931348623157e308,
            f64::POSITIVE_INFINITY,
        ],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        f64::NEGATIVE_INFINITY,
        f64::NEGATIVE_INFINITY,
        &[f64::NEGATIVE_INFINITY],
        &[f64::NEGATIVE_INFINITY],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(
        f64::POSITIVE_INFINITY,
        f64::POSITIVE_INFINITY,
        &[f64::POSITIVE_INFINITY],
        &[f64::POSITIVE_INFINITY],
    );
    primitive_float_increasing_inclusive_range_helper::<f64>(0.0, 0.0, &[0.0], &[0.0]);
    primitive_float_increasing_inclusive_range_helper::<f64>(-0.0, -0.0, &[-0.0], &[-0.0]);
    primitive_float_increasing_inclusive_range_helper::<f64>(-0.0, 0.0, &[-0.0, 0.0], &[-0.0, 0.0]);
}

fn primitive_float_increasing_inclusive_range_fail_helper<T: PrimitiveFloat>() {
    assert_panic!(primitive_float_increasing_inclusive_range::<T>(
        T::ONE,
        T::ZERO
    ));
    assert_panic!(primitive_float_increasing_inclusive_range::<T>(
        T::ONE,
        T::NAN
    ));
}

#[test]
fn primitive_float_increasing_inclusive_range_fail() {
    apply_fn_to_primitive_floats!(primitive_float_increasing_inclusive_range_fail_helper);
}
