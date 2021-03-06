use malachite_base::iterators::bit_distributor::BitDistributorOutputType;
use malachite_base::num::arithmetic::traits::PowerOfTwo;
use malachite_base::num::basic::integers::PrimitiveInt;
use malachite_base::num::basic::traits::Two;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base::num::conversion::traits::{ExactFrom, SaturatingFrom, WrappingFrom};
use malachite_base::num::exhaustive::{
    exhaustive_positive_primitive_ints, exhaustive_unsigneds,
    primitive_int_increasing_inclusive_range, primitive_int_increasing_range,
};
use malachite_base::num::iterators::{bit_distributor_sequence, ruler_sequence};
use malachite_base::tuples::exhaustive::{
    exhaustive_dependent_pairs, exhaustive_pairs, exhaustive_triples_from_single, lex_pairs,
    ExhaustiveDependentPairsYsGenerator,
};
use malachite_base::vecs::exhaustive::{
    exhaustive_fixed_length_vecs_from_single, exhaustive_vecs, exhaustive_vecs_length_range,
    exhaustive_vecs_min_length,
};
use malachite_base_test_util::generators::common::{
    permute_1_3_2, permute_2_1, reshape_2_1_to_3, It,
};
use malachite_base_test_util::generators::exhaustive::UnsignedVecTripleLenGenerator;
use malachite_base_test_util::generators::exhaustive_pairs_big_tiny;
use malachite_nz::integer::exhaustive::exhaustive_integers;
use malachite_nz::integer::Integer;
use malachite_nz::natural::arithmetic::mul::fft::*;
use malachite_nz::natural::arithmetic::mul::toom::{
    _limbs_mul_greater_to_out_toom_22_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_32_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_33_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_42_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_43_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_44_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_52_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_53_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_54_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_62_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_63_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_6h_input_sizes_valid,
    _limbs_mul_greater_to_out_toom_8h_input_sizes_valid,
};
use malachite_nz::natural::conversion::digits::general_digits::{
    limbs_digit_count, limbs_per_digit_in_base, GET_STR_PRECOMPUTE_THRESHOLD,
};
use malachite_nz::natural::exhaustive::{
    exhaustive_natural_range_to_infinity, exhaustive_naturals,
};
use malachite_nz::natural::Natural;
use malachite_nz::platform::Limb;
use std::iter::once;
use std::marker::PhantomData;

// -- Integer --

pub fn exhaustive_integer_gen() -> It<Integer> {
    Box::new(exhaustive_integers())
}

// -- (Integer, PrimitiveUnsigned) --

pub fn exhaustive_integer_unsigned_pair_gen_var_1<T: ExactFrom<u8> + PrimitiveUnsigned>(
) -> It<(Integer, T)> {
    Box::new(lex_pairs(
        exhaustive_integers(),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))
}

pub fn exhaustive_integer_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Integer, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_integers(),
        exhaustive_unsigneds(),
    ))
}

// -- (Integer, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_integer_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Integer, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_integers(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))))
}

// -- Natural --

pub fn exhaustive_natural_gen() -> It<Natural> {
    Box::new(exhaustive_naturals())
}

pub fn exhaustive_natural_gen_var_1() -> It<Natural> {
    Box::new(exhaustive_natural_range_to_infinity(Natural::TWO))
}

// -- (Natural, Natural) --

pub fn exhaustive_natural_pair_gen_var_1() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_natural_range_to_infinity(Natural::power_of_two(Limb::WIDTH)),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

pub fn exhaustive_natural_pair_gen_var_2() -> It<(Natural, Natural)> {
    Box::new(exhaustive_pairs(
        exhaustive_naturals(),
        exhaustive_natural_range_to_infinity(Natural::TWO),
    ))
}

// -- (Natural, PrimitiveInt) --

pub fn exhaustive_natural_primitive_int_pair_gen_var_1<
    T: PrimitiveInt + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

pub fn exhaustive_natural_primitive_int_pair_gen_var_2<T: PrimitiveInt>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::MAX),
    ))
}

// -- (Natural, PrimitiveUnsigned) --

pub fn exhaustive_natural_unsigned_pair_gen_var_1<T: ExactFrom<u8> + PrimitiveUnsigned>(
) -> It<(Natural, T)> {
    Box::new(lex_pairs(
        exhaustive_naturals(),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))
}

pub fn exhaustive_natural_unsigned_pair_gen_var_2<T: PrimitiveUnsigned>() -> It<(Natural, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_naturals(),
        exhaustive_unsigneds(),
    ))
}

// -- (Natural, PrimitiveUnsigned, PrimitiveUnsigned) --

pub fn exhaustive_natural_unsigned_unsigned_triple_gen_var_1<
    T: ExactFrom<u8> + PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Natural, T, U)> {
    permute_1_3_2(reshape_2_1_to_3(Box::new(lex_pairs(
        exhaustive_pairs_big_tiny(exhaustive_naturals(), exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::exact_from(36u8)),
    ))))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned)

pub fn exhaustive_unsigned_vec_unsigned_pair_gen_var_1<
    T: PrimitiveUnsigned + SaturatingFrom<U>,
    U: PrimitiveInt,
>() -> It<(Vec<T>, T)> {
    Box::new(exhaustive_pairs_big_tiny(
        exhaustive_vecs_min_length(2, exhaustive_unsigneds()),
        primitive_int_increasing_inclusive_range(T::TWO, T::saturating_from(U::MAX)),
    ))
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>)

struct ValidLengthsGenerator;

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), Vec<T>, It<Vec<T>>>
    for ValidLengthsGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            limbs_digit_count(&p.0, p.1),
            exhaustive_unsigneds(),
        ))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_triple_gen_var_1<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, u64, Vec<Limb>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs(exhaustive_unsigneds()),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            ValidLengthsGenerator,
        )
        .map(|((xs, base), out)| (out, base, xs)),
    )
}

// -- (Vec<PrimitiveUnsigned>, PrimitiveUnsigned, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidLengthsBasecaseGenerator {
    min_out_len: usize,
}

impl<T: PrimitiveUnsigned> ExhaustiveDependentPairsYsGenerator<usize, Vec<T>, It<Vec<T>>>
    for ValidLengthsBasecaseGenerator
{
    #[inline]
    fn get_ys(&self, &len: &usize) -> It<Vec<T>> {
        Box::new(exhaustive_vecs_min_length(
            u64::exact_from(if len == 0 { self.min_out_len } else { len }),
            exhaustive_unsigneds(),
        ))
    }
}

struct BasecaseDigitsInputGenerator;

impl<T: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(Vec<Limb>, u64), (Vec<T>, usize), It<(Vec<T>, usize)>>
    for BasecaseDigitsInputGenerator
{
    #[inline]
    fn get_ys(&self, p: &(Vec<Limb>, u64)) -> It<(Vec<T>, usize)> {
        let min_out_len = usize::exact_from(limbs_digit_count(&p.0, p.1));
        permute_2_1(Box::new(exhaustive_dependent_pairs(
            ruler_sequence(),
            once(0).chain(primitive_int_increasing_inclusive_range(
                min_out_len,
                usize::MAX,
            )),
            ValidLengthsBasecaseGenerator { min_out_len },
        )))
    }
}

pub fn exhaustive_unsigned_vec_unsigned_unsigned_vec_unsigned_quadruple_gen_var_1<
    T: PrimitiveUnsigned,
>() -> It<(Vec<T>, usize, Vec<Limb>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                exhaustive_vecs_length_range(
                    0,
                    u64::wrapping_from(GET_STR_PRECOMPUTE_THRESHOLD),
                    exhaustive_unsigneds(),
                ),
                (3u64..256).filter(|&b| !b.is_power_of_two()),
            ),
            BasecaseDigitsInputGenerator,
        )
        .map(|((xs, base), (out, len))| (out, len, xs, base)),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, PrimitiveUnsigned) --

struct ValidDigitsGenerator<T: PrimitiveUnsigned, U: PrimitiveUnsigned> {
    phantom_t: PhantomData<*const T>,
    phantom_u: PhantomData<*const U>,
}

impl<T: PrimitiveUnsigned, U: PrimitiveUnsigned>
    ExhaustiveDependentPairsYsGenerator<(u64, usize), (Vec<T>, Vec<U>), It<(Vec<T>, Vec<U>)>>
    for ValidDigitsGenerator<T, U>
{
    #[inline]
    fn get_ys(&self, p: &(u64, usize)) -> It<(Vec<T>, Vec<U>)> {
        Box::new(exhaustive_pairs(
            exhaustive_fixed_length_vecs_from_single(
                u64::wrapping_from(p.1),
                primitive_int_increasing_range(T::ZERO, T::wrapping_from(p.0)),
            ),
            exhaustive_vecs_min_length(limbs_per_digit_in_base(p.1, p.0), exhaustive_unsigneds()),
        ))
    }
}

// var 1 is in malachite-base

pub fn exhaustive_unsigned_vec_unsigned_vec_unsigned_triple_gen_var_2<
    T: PrimitiveUnsigned,
    U: PrimitiveUnsigned,
>() -> It<(Vec<U>, Vec<T>, u64)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::normal(1),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_pairs_big_tiny(
                (3u64..256).filter(|&b| !b.is_power_of_two()),
                exhaustive_positive_primitive_ints(),
            ),
            ValidDigitsGenerator {
                phantom_t: PhantomData,
                phantom_u: PhantomData,
            },
        )
        .map(|((base, _), (xs, out))| (out, xs, base)),
    )
}

// -- (Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>, Vec<PrimitiveUnsigned>) --

// vars 1 through 3 are in malachite-base

fn exhaustive_mul_helper<T: PrimitiveUnsigned, F: Fn(usize, usize) -> bool>(
    valid: &'static F,
    min_x: u64,
    min_y: u64,
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    Box::new(
        exhaustive_dependent_pairs(
            bit_distributor_sequence(
                BitDistributorOutputType::tiny(),
                BitDistributorOutputType::normal(1),
            ),
            exhaustive_triples_from_single(exhaustive_unsigneds::<u64>()).flat_map(
                move |(o, x, y)| {
                    let x = x.checked_add(min_x)?;
                    let y = y.checked_add(min_y)?;
                    if valid(usize::exact_from(x), usize::exact_from(y)) {
                        let o = x.checked_add(y)?.checked_add(o)?;
                        Some((o, x, y))
                    } else {
                        None
                    }
                },
            ),
            UnsignedVecTripleLenGenerator,
        )
        .map(|p| p.1),
    )
}

pub fn exhaustive_unsigned_vec_triple_gen_var_4<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_22_input_sizes_valid, 2, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_5<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_32_input_sizes_valid, 6, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_6<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_33_input_sizes_valid, 3, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_7<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_42_input_sizes_valid, 4, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_8<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_43_input_sizes_valid, 11, 8)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_9<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_44_input_sizes_valid, 4, 4)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_10<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_52_input_sizes_valid, 14, 5)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_11<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_53_input_sizes_valid, 5, 3)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_12<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_54_input_sizes_valid, 14, 11)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_13<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_62_input_sizes_valid, 6, 2)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_14<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_63_input_sizes_valid, 17, 9)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_15<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_6h_input_sizes_valid, 42, 42)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_16<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_toom_8h_input_sizes_valid, 86, 86)
}

pub fn exhaustive_unsigned_vec_triple_gen_var_17<T: PrimitiveUnsigned>(
) -> It<(Vec<T>, Vec<T>, Vec<T>)> {
    exhaustive_mul_helper(&_limbs_mul_greater_to_out_fft_input_sizes_threshold, 15, 15)
}