use crate::num::conversion::traits::ExactFrom;
use crate::num::random::geometric::{
    geometric_random_unsigned_inclusive_range, geometric_random_unsigneds,
    GeometricRandomNaturalValues,
};
use crate::num::random::{
    random_unsigned_inclusive_range, random_unsigned_range, RandomUnsignedInclusiveRange,
    RandomUnsignedRange,
};
use crate::random::Seed;
#[cfg(not(feature = "test_build"))]
use alloc::collections::BTreeSet;
use core::hash::Hash;
#[cfg(not(feature = "test_build"))]
use hashbrown::HashSet;
#[cfg(feature = "test_build")]
use std::collections::{BTreeSet, HashSet};

/// Generates random [`HashSet`]s of a fixed length, where the [`Vec`]s have no repeated elements,
/// and the elements are in ascending order.
///
/// This `struct` is created by [`random_hash_sets_fixed_length`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomHashSetsFixedLength<I: Iterator>
where
    I::Item: Eq + Hash,
{
    len: usize,
    xs: I,
}

impl<I: Iterator> Iterator for RandomHashSetsFixedLength<I>
where
    I::Item: Eq + Hash,
{
    type Item = HashSet<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<HashSet<I::Item>> {
        let mut set = HashSet::new();
        while set.len() < self.len {
            set.insert(self.xs.next().unwrap());
        }
        Some(set)
    }
}

/// Randomly generates [`HashSet`]s of a given length.
///
/// The input iterator must generate at least `len` distinct elements; otherwise, this iterator
/// will hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// If `len` is 0, the output consists of the empty set, repeated.
///
/// `xs` must be infinite.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets_fixed_length;
///
/// fn main() {
///     let xss = random_hash_sets_fixed_length(
///         2,
///         random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100),
///     )
///     .take(10)
///     .collect_vec();
///     assert_eq!(
///         xss,
///         &[
///             hashset!{24, 95},
///             hashset!{71, 99},
///             hashset!{53, 93},
///             hashset!{34, 85},
///             hashset!{2, 48},
///             hashset!{11, 55},
///             hashset!{18, 48},
///             hashset!{90, 93},
///             hashset!{67, 93},
///             hashset!{93, 95}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets_fixed_length<I: Iterator>(len: u64, xs: I) -> RandomHashSetsFixedLength<I>
where
    I::Item: Eq + Hash,
{
    RandomHashSetsFixedLength {
        len: usize::exact_from(len),
        xs,
    }
}

/// Generates random [`HashSet`]s with lengths from an iterator.
#[derive(Clone, Debug)]
pub struct RandomHashSets<T: Eq + Hash, I: Iterator<Item = u64>, J: Iterator<Item = T>> {
    lengths: I,
    xs: J,
}

impl<T: Eq + Hash, I: Iterator<Item = u64>, J: Iterator<Item = T>> Iterator
    for RandomHashSets<T, I, J>
{
    type Item = HashSet<T>;

    fn next(&mut self) -> Option<HashSet<T>> {
        let len = usize::exact_from(self.lengths.next().unwrap());
        let mut set = HashSet::new();
        while set.len() < len {
            set.insert(self.xs.next().unwrap());
        }
        Some(set)
    }
}

/// Generates random [`HashSet`]s using elements from an iterator and with lengths from another
/// iterator.
///
/// The input iterator must generate at least many distinct elements as any number generated by the
/// lengths iterator; otherwise, this iterator will hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P(n)\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `lengths` and `xs` must be infinite.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets_from_length_iterator;
/// use malachite_base::vecs::random_values_from_vec;
///
/// fn main() {
///     let xs = random_hash_sets_from_length_iterator(
///         EXAMPLE_SEED,
///         &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///         &random_primitive_ints::<u8>,
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             hashset!{11, 85},
///             hashset!{134, 136, 200, 235},
///             hashset!{203, 223},
///             hashset!{38, 177, 217, 235},
///             hashset!{32, 162, 166, 234},
///             hashset!{30, 218},
///             hashset!{},
///             hashset!{90, 106},
///             hashset!{},
///             hashset!{9, 151, 204, 216},
///             hashset!{78, 97, 213, 253},
///             hashset!{39, 91},
///             hashset!{170, 175, 191, 232},
///             hashset!{2, 233},
///             hashset!{22, 35, 198, 217},
///             hashset!{17, 32, 114, 173},
///             hashset!{65, 114, 121, 222},
///             hashset!{},
///             hashset!{25, 144, 148, 173},
///             hashset!{}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets_from_length_iterator<
    T: Eq + Hash,
    I: Iterator<Item = u64>,
    J: Iterator<Item = T>,
>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
) -> RandomHashSets<T, I, J> {
    RandomHashSets {
        lengths: lengths_gen(seed.fork("lengths")),
        xs: xs_gen(seed.fork("xs")),
    }
}

/// Generates random [`HashSet`]s using elements from an iterator.
///
/// The lengths of the [`HashSet`]s are sampled from a geometric distribution with a specified mean
/// $m$, equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if
/// `mean_length_numerator / mean_length_denominator` is significantly lower than 256, then it's
/// ok to use `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in [`geometric_random_unsigneds`].
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets;
///
/// fn main() {
///     let xs = random_hash_sets(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             hashset!{},
///             hashset!{11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235},
///             hashset!{30, 90, 218, 234},
///             hashset!{9, 106, 204, 216},
///             hashset!{151},
///             hashset!{},
///             hashset!{78, 91, 97, 213, 253},
///             hashset!{39, 191},
///             hashset!{170, 175, 232, 233},
///             hashset!{},
///             hashset!{2, 22, 35, 114, 198, 217},
///             hashset!{},
///             hashset!{},
///             hashset!{17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222},
///             hashset!{52, 69, 73, 91, 115, 137, 153, 178},
///             hashset!{},
///             hashset!{34, 95, 112},
///             hashset!{},
///             hashset!{106, 130, 167, 168, 197},
///             hashset!{86, 101, 122, 150, 172, 177, 207, 218, 221}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomHashSets<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_hash_sets_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        xs_gen,
    )
}

/// Generates random [`HashSet`]s with a minimum length, using elements from an iterator.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if
/// `mean_length_numerator / mean_length_denominator` is significantly lower than 256, then it's
/// ok to use `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in
/// [`geometric_random_unsigned_inclusive_range`] with $a$ equal to `min_length` and `b` to
/// `u64::MAX`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, if their ratio is less
/// than or equal to `min_length`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets_min_length;
///
/// fn main() {
///     let xs = random_hash_sets_min_length(
///         EXAMPLE_SEED,
///         2,
///         &random_primitive_ints::<u8>,
///         6,
///         1
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             hashset!{11, 85},
///             hashset!{
///                 30, 32, 38, 90, 134, 136, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235
///             },
///             hashset!{9, 106, 151, 204, 213, 216},
///             hashset!{39, 78, 91, 97, 191, 253},
///             hashset!{170, 175, 232},
///             hashset!{2, 233},
///             hashset!{17, 22, 32, 35, 114, 198, 217},
///             hashset!{65, 114, 121, 173},
///             hashset!{25, 79, 144, 148, 173, 222},
///             hashset!{52, 115},
///             hashset!{34, 69, 73, 91, 112, 137, 153, 178},
///             hashset!{95, 106},
///             hashset!{167, 197},
///             hashset!{74, 86, 101, 115, 122, 130, 150, 168, 172, 177, 207, 218, 221},
///             hashset!{9, 48, 52, 109, 123, 133, 159, 201, 247, 250},
///             hashset!{196, 235},
///             hashset!{40, 68, 97, 104, 190},
///             hashset!{7, 216},
///             hashset!{11, 24, 43, 112, 157, 216, 217},
///             hashset!{29, 51, 55, 65, 84, 89, 103, 135, 191, 206, 211}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets_min_length<I: Iterator>(
    seed: Seed,
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomHashSets<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_hash_sets_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        xs_gen,
    )
}

/// Generates random [`HashSet`]s with lengths in $[a, b)$, using elements from an iterator.
///
/// The lengths of the [`HashSet`]s are sampled from a uniform distribution on $[a, b)$. $a$ must be
/// less than $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets_length_range;
///
/// fn main() {
///     let xs = random_hash_sets_length_range(
///         EXAMPLE_SEED,
///         2,
///         5,
///         &random_primitive_ints::<u8>
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             hashset!{11, 85, 136},
///             hashset!{134, 200, 203, 235},
///             hashset!{38, 223, 235},
///             hashset!{32, 162, 177, 217},
///             hashset!{30, 166, 218, 234},
///             hashset!{9, 90, 106},
///             hashset!{204, 216},
///             hashset!{97, 151, 213},
///             hashset!{78, 253},
///             hashset!{39, 91, 175, 191},
///             hashset!{2, 170, 232, 233},
///             hashset!{22, 35, 217},
///             hashset!{17, 32, 114, 198},
///             hashset!{65, 114, 173},
///             hashset!{25, 121, 173, 222},
///             hashset!{79, 115, 144, 148},
///             hashset!{52, 69, 73, 137},
///             hashset!{91, 153},
///             hashset!{34, 95, 112, 178},
///             hashset!{106, 167}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets_length_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomHashSets<I::Item, RandomUnsignedRange<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_hash_sets_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        xs_gen,
    )
}

/// Generates random [`HashSet`]s with lengths in $[a, b]$, using elements from an iterator.
///
/// The lengths of the [`HashSet`]s are sampled from a uniform distribution on $[a, b)$. $a$ must be
/// less than or equal to $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a + 1}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_hash_sets_length_inclusive_range;
///
/// fn main() {
///     let xs = random_hash_sets_length_inclusive_range(
///         EXAMPLE_SEED,
///         2,
///         4,
///         &random_primitive_ints::<u8>
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             hashset!{11, 85, 136},
///             hashset!{134, 200, 203, 235},
///             hashset!{38, 223, 235},
///             hashset!{32, 162, 177, 217},
///             hashset!{30, 166, 218, 234},
///             hashset!{9, 90, 106},
///             hashset!{204, 216},
///             hashset!{97, 151, 213},
///             hashset!{78, 253},
///             hashset!{39, 91, 175, 191},
///             hashset!{2, 170, 232, 233},
///             hashset!{22, 35, 217},
///             hashset!{17, 32, 114, 198},
///             hashset!{65, 114, 173},
///             hashset!{25, 121, 173, 222},
///             hashset!{79, 115, 144, 148},
///             hashset!{52, 69, 73, 137},
///             hashset!{91, 153},
///             hashset!{34, 95, 112, 178},
///             hashset!{106, 167}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_hash_sets_length_inclusive_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomHashSets<I::Item, RandomUnsignedInclusiveRange<u64>, I>
where
    I::Item: Eq + Hash,
{
    random_hash_sets_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        xs_gen,
    )
}

/// Generates random [`BTreeSet`]s of a fixed length, where the [`Vec`]s have no repeated elements,
/// and the elements are in ascending order.
///
/// This `struct` is created by [`random_b_tree_sets_fixed_length`]; see its documentation for more.
#[derive(Clone, Debug)]
pub struct RandomBTreeSetsFixedLength<I: Iterator>
where
    I::Item: Ord,
{
    len: usize,
    xs: I,
}

impl<I: Iterator> Iterator for RandomBTreeSetsFixedLength<I>
where
    I::Item: Ord,
{
    type Item = BTreeSet<I::Item>;

    #[inline]
    fn next(&mut self) -> Option<BTreeSet<I::Item>> {
        let mut set = BTreeSet::new();
        while set.len() < self.len {
            set.insert(self.xs.next().unwrap());
        }
        Some(set)
    }
}

/// Randomly generates [`BTreeSet`]s of a given length.
///
/// The input iterator must generate at least `len` distinct elements; otherwise, this iterator
/// will hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// If `len` is 0, the output consists of the empty set, repeated.
///
/// `xs` must be infinite.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_unsigned_inclusive_range;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets_fixed_length;
///
/// fn main() {
///     let xss = random_b_tree_sets_fixed_length(
///         2,
///         random_unsigned_inclusive_range::<u32>(EXAMPLE_SEED, 1, 100),
///     )
///     .take(10)
///     .collect_vec();
///     assert_eq!(
///         xss,
///         &[
///             btreeset!{24, 95},
///             btreeset!{71, 99},
///             btreeset!{53, 93},
///             btreeset!{34, 85},
///             btreeset!{2, 48},
///             btreeset!{11, 55},
///             btreeset!{18, 48},
///             btreeset!{90, 93},
///             btreeset!{67, 93},
///             btreeset!{93, 95}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets_fixed_length<I: Iterator>(
    len: u64,
    xs: I,
) -> RandomBTreeSetsFixedLength<I>
where
    I::Item: Ord,
{
    RandomBTreeSetsFixedLength {
        len: usize::exact_from(len),
        xs,
    }
}

/// Generates random [`BTreeSet`]s with lengths from an iterator.
#[derive(Clone, Debug)]
pub struct RandomBTreeSets<T: Ord, I: Iterator<Item = u64>, J: Iterator<Item = T>> {
    lengths: I,
    xs: J,
}

impl<T: Ord, I: Iterator<Item = u64>, J: Iterator<Item = T>> Iterator for RandomBTreeSets<T, I, J> {
    type Item = BTreeSet<T>;

    fn next(&mut self) -> Option<BTreeSet<T>> {
        let len = usize::exact_from(self.lengths.next().unwrap());
        let mut set = BTreeSet::new();
        while set.len() < len {
            set.insert(self.xs.next().unwrap());
        }
        Some(set)
    }
}

/// Generates random [`BTreeSet`]s using elements from an iterator and with lengths from another
/// iterator.
///
/// The input iterator must generate at least many distinct elements as any number generated by the
/// lengths iterator; otherwise, this iterator will hang.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P(n)\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `lengths` and `xs` must be infinite.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets_from_length_iterator;
/// use malachite_base::vecs::random_values_from_vec;
///
/// fn main() {
///     let xs = random_b_tree_sets_from_length_iterator(
///         EXAMPLE_SEED,
///         &|seed| random_values_from_vec(seed, vec![0, 2, 4]),
///         &random_primitive_ints::<u8>,
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             btreeset!{11, 85},
///             btreeset!{134, 136, 200, 235},
///             btreeset!{203, 223},
///             btreeset!{38, 177, 217, 235},
///             btreeset!{32, 162, 166, 234},
///             btreeset!{30, 218},
///             btreeset!{},
///             btreeset!{90, 106},
///             btreeset!{},
///             btreeset!{9, 151, 204, 216},
///             btreeset!{78, 97, 213, 253},
///             btreeset!{39, 91},
///             btreeset!{170, 175, 191, 232},
///             btreeset!{2, 233},
///             btreeset!{22, 35, 198, 217},
///             btreeset!{17, 32, 114, 173},
///             btreeset!{65, 114, 121, 222},
///             btreeset!{},
///             btreeset!{25, 144, 148, 173},
///             btreeset!{}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets_from_length_iterator<
    T: Ord,
    I: Iterator<Item = u64>,
    J: Iterator<Item = T>,
>(
    seed: Seed,
    lengths_gen: &dyn Fn(Seed) -> I,
    xs_gen: &dyn Fn(Seed) -> J,
) -> RandomBTreeSets<T, I, J> {
    RandomBTreeSets {
        lengths: lengths_gen(seed.fork("lengths")),
        xs: xs_gen(seed.fork("xs")),
    }
}

/// Generates random [`BTreeSet`]s using elements from an iterator.
///
/// The lengths of the [`BTreeSet`]s are sampled from a geometric distribution with a specified mean
/// $m$, equal to `mean_length_numerator / mean_length_denominator`. $m$ must be greater than 0.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if
/// `mean_length_numerator / mean_length_denominator` is significantly lower than 256, then it's
/// ok to use `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in [`geometric_random_unsigneds`].
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, or, if after being
/// reduced to lowest terms, their sum is greater than or equal to $2^{64}$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets;
///
/// fn main() {
///     let xs = random_b_tree_sets(EXAMPLE_SEED, &random_primitive_ints::<u8>, 4, 1);
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             btreeset!{},
///             btreeset!{11, 32, 38, 85, 134, 136, 162, 166, 177, 200, 203, 217, 223, 235},
///             btreeset!{30, 90, 218, 234},
///             btreeset!{9, 106, 204, 216},
///             btreeset!{151},
///             btreeset!{},
///             btreeset!{78, 91, 97, 213, 253},
///             btreeset!{39, 191},
///             btreeset!{170, 175, 232, 233},
///             btreeset!{},
///             btreeset!{2, 22, 35, 114, 198, 217},
///             btreeset!{},
///             btreeset!{},
///             btreeset!{17, 25, 32, 65, 79, 114, 121, 144, 148, 173, 222},
///             btreeset!{52, 69, 73, 91, 115, 137, 153, 178},
///             btreeset!{},
///             btreeset!{34, 95, 112},
///             btreeset!{},
///             btreeset!{106, 130, 167, 168, 197},
///             btreeset!{86, 101, 122, 150, 172, 177, 207, 218, 221}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets<I: Iterator>(
    seed: Seed,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomBTreeSets<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Ord,
{
    random_b_tree_sets_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigneds(seed_2, mean_length_numerator, mean_length_denominator)
        },
        xs_gen,
    )
}

/// Generates random [`BTreeSet`]s with a minimum length, using elements from an iterator.
///
/// Strictly speaking, the input iterator must generate infinitely many distinct elements. In
/// practice it only needs to generate $k$ distinct elements, where $k$ is the largest length
/// actually sampled from the geometric distribution. For example, if
/// `mean_length_numerator / mean_length_denominator` is significantly lower than 256, then it's
/// ok to use `random_unsigneds::<u8>`.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}) = n!P_g(n)\prod\_{i=0}^{n-1}P(x\_i),
/// $$
/// where $P_g(n)$ is the probability function described in
/// [`geometric_random_unsigned_inclusive_range`], with $a$ equal to `min_length` and `b` to
/// `u64::MAX`.
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if `mean_length_numerator` or `mean_length_denominator` are zero, if their ratio is less
/// than or equal to `min_length`, or if they are too large and manipulating them leads to
/// arithmetic overflow.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets_min_length;
///
/// fn main() {
///     let xs = random_b_tree_sets_min_length(
///         EXAMPLE_SEED,
///         2,
///         &random_primitive_ints::<u8>,
///         6,
///         1
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             btreeset!{11, 85},
///             btreeset!{
///                 30, 32, 38, 90, 134, 136, 162, 166, 177, 200, 203, 217, 218, 223, 234, 235
///             },
///             btreeset!{9, 106, 151, 204, 213, 216},
///             btreeset!{39, 78, 91, 97, 191, 253},
///             btreeset!{170, 175, 232},
///             btreeset!{2, 233},
///             btreeset!{17, 22, 32, 35, 114, 198, 217},
///             btreeset!{65, 114, 121, 173},
///             btreeset!{25, 79, 144, 148, 173, 222},
///             btreeset!{52, 115},
///             btreeset!{34, 69, 73, 91, 112, 137, 153, 178},
///             btreeset!{95, 106},
///             btreeset!{167, 197},
///             btreeset!{74, 86, 101, 115, 122, 130, 150, 168, 172, 177, 207, 218, 221},
///             btreeset!{9, 48, 52, 109, 123, 133, 159, 201, 247, 250},
///             btreeset!{196, 235},
///             btreeset!{40, 68, 97, 104, 190},
///             btreeset!{7, 216},
///             btreeset!{11, 24, 43, 112, 157, 216, 217},
///             btreeset!{29, 51, 55, 65, 84, 89, 103, 135, 191, 206, 211}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets_min_length<I: Iterator>(
    seed: Seed,
    min_length: u64,
    xs_gen: &dyn Fn(Seed) -> I,
    mean_length_numerator: u64,
    mean_length_denominator: u64,
) -> RandomBTreeSets<I::Item, GeometricRandomNaturalValues<u64>, I>
where
    I::Item: Ord,
{
    random_b_tree_sets_from_length_iterator(
        seed,
        &|seed_2| {
            geometric_random_unsigned_inclusive_range(
                seed_2,
                min_length,
                u64::MAX,
                mean_length_numerator,
                mean_length_denominator,
            )
        },
        xs_gen,
    )
}

/// Generates random [`BTreeSet`]s with lengths in $[a, b]$, using elements from an iterator.
///
/// The lengths of the [`BTreeSet`]s are sampled from a uniform distribution on $[a, b)$. $a$ must
/// be less than or equal to $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a + 1}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets_length_range;
///
/// fn main() {
///     let xs = random_b_tree_sets_length_range(
///         EXAMPLE_SEED,
///         2,
///         5,
///         &random_primitive_ints::<u8>
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             btreeset!{11, 85, 136},
///             btreeset!{134, 200, 203, 235},
///             btreeset!{38, 223, 235},
///             btreeset!{32, 162, 177, 217},
///             btreeset!{30, 166, 218, 234},
///             btreeset!{9, 90, 106},
///             btreeset!{204, 216},
///             btreeset!{97, 151, 213},
///             btreeset!{78, 253},
///             btreeset!{39, 91, 175, 191},
///             btreeset!{2, 170, 232, 233},
///             btreeset!{22, 35, 217},
///             btreeset!{17, 32, 114, 198},
///             btreeset!{65, 114, 173},
///             btreeset!{25, 121, 173, 222},
///             btreeset!{79, 115, 144, 148},
///             btreeset!{52, 69, 73, 137},
///             btreeset!{91, 153},
///             btreeset!{34, 95, 112, 178},
///             btreeset!{106, 167}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets_length_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomBTreeSets<I::Item, RandomUnsignedRange<u64>, I>
where
    I::Item: Ord,
{
    random_b_tree_sets_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_range(seed_2, a, b),
        xs_gen,
    )
}

/// Generates random [`BTreeSet`]s with lengths in $[a, b]$, using elements from an iterator.
///
/// The lengths of the [`BTreeSet`]s are sampled from a uniform distribution on $[a, b)$. $a$ must
/// be less than or equal to $b$.
///
/// The input iterator must generate at least $b$ distinct elements.
///
/// $$
/// P((x\_i)\_{i=0}^{n-1}, a, b) = \frac{n!}{b - a + 1}\prod\_{i=0}^{n-1}P(x\_i).
/// $$
///
/// `xs_gen` must be infinite.
///
/// # Panics
/// Panics if $a \geq b$.
///
/// # Examples
/// ```
/// #[macro_use]
/// extern crate maplit;
///
/// use itertools::Itertools;
/// use malachite_base::num::random::random_primitive_ints;
/// use malachite_base::random::EXAMPLE_SEED;
/// use malachite_base::sets::random::random_b_tree_sets_length_inclusive_range;
///
/// fn main() {
///     let xs = random_b_tree_sets_length_inclusive_range(
///         EXAMPLE_SEED,
///         2,
///         4,
///         &random_primitive_ints::<u8>
///     );
///     let values = xs.take(20).collect_vec();
///     assert_eq!(
///         values,
///         &[
///             btreeset!{11, 85, 136},
///             btreeset!{134, 200, 203, 235},
///             btreeset!{38, 223, 235},
///             btreeset!{32, 162, 177, 217},
///             btreeset!{30, 166, 218, 234},
///             btreeset!{9, 90, 106},
///             btreeset!{204, 216},
///             btreeset!{97, 151, 213},
///             btreeset!{78, 253},
///             btreeset!{39, 91, 175, 191},
///             btreeset!{2, 170, 232, 233},
///             btreeset!{22, 35, 217},
///             btreeset!{17, 32, 114, 198},
///             btreeset!{65, 114, 173},
///             btreeset!{25, 121, 173, 222},
///             btreeset!{79, 115, 144, 148},
///             btreeset!{52, 69, 73, 137},
///             btreeset!{91, 153},
///             btreeset!{34, 95, 112, 178},
///             btreeset!{106, 167}
///         ]
///     );
/// }
/// ```
#[inline]
pub fn random_b_tree_sets_length_inclusive_range<I: Iterator>(
    seed: Seed,
    a: u64,
    b: u64,
    xs_gen: &dyn Fn(Seed) -> I,
) -> RandomBTreeSets<I::Item, RandomUnsignedInclusiveRange<u64>, I>
where
    I::Item: Ord,
{
    random_b_tree_sets_from_length_iterator(
        seed,
        &|seed_2| random_unsigned_inclusive_range(seed_2, a, b),
        xs_gen,
    )
}
