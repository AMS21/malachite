use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::conversion::traits::ExactFrom;
use malachite_base::num::exhaustive::exhaustive_signeds;

fn exhaustive_signeds_helper<T: PrimitiveSigned>()
where
    i8: ExactFrom<T>,
{
    assert_eq!(
        exhaustive_signeds::<T>()
            .map(i8::exact_from)
            .take(20)
            .collect::<Vec<i8>>(),
        &[0, 1, -1, 2, -2, 3, -3, 4, -4, 5, -5, 6, -6, 7, -7, 8, -8, 9, -9, 10]
    );
}

fn exhaustive_signeds_long_helper<T: PrimitiveSigned>(last_20: &[T]) {
    let expected_len = usize::power_of_two(T::WIDTH);
    let xs = exhaustive_signeds::<T>();
    assert_eq!(xs.clone().count(), expected_len);
    assert_eq!(xs.skip(expected_len - 20).collect::<Vec<T>>(), last_20)
}

#[test]
fn test_exhaustive_signeds() {
    exhaustive_signeds_helper::<i8>();
    exhaustive_signeds_helper::<i16>();
    exhaustive_signeds_helper::<i32>();
    exhaustive_signeds_helper::<i64>();
    exhaustive_signeds_helper::<i128>();
    exhaustive_signeds_helper::<isize>();

    exhaustive_signeds_long_helper::<i8>(&[
        -118, 119, -119, 120, -120, 121, -121, 122, -122, 123, -123, 124, -124, 125, -125, 126,
        -126, 127, -127, -128,
    ]);
    exhaustive_signeds_long_helper::<i16>(&[
        -0x7ff6,
        0x7ff7,
        -0x7ff7,
        0x7ff8,
        -0x7ff8,
        0x7ff9,
        -0x7ff9,
        0x7ffa,
        -0x7ffa,
        0x7ffb,
        -0x7ffb,
        0x7ffc,
        -0x7ffc,
        0x7ffd,
        -0x7ffd,
        i16::MAX - 1,
        -0x7ffe,
        i16::MAX,
        i16::MIN + 1,
        i16::MIN,
    ]);
}