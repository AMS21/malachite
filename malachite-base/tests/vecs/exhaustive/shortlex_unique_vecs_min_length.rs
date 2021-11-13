use itertools::Itertools;
use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::chars::exhaustive::exhaustive_ascii_chars;
use malachite_base::nevers::nevers;
use malachite_base::tuples::exhaustive::exhaustive_units;
use malachite_base::vecs::exhaustive::shortlex_unique_vecs_min_length;
use std::fmt::Debug;

fn shortlex_unique_vecs_min_length_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = shortlex_unique_vecs_min_length(min_length, xs)
        .take(20)
        .collect_vec();
    assert_eq!(xss.iter().map(Vec::as_slice).collect_vec().as_slice(), out);
}

fn shortlex_unique_vecs_min_length_small_helper<I: Clone + Iterator>(
    min_length: u64,
    xs: I,
    out_len: usize,
    out: &[&[I::Item]],
) where
    I::Item: Clone + Debug + Eq,
{
    let xss = shortlex_unique_vecs_min_length(min_length, xs);
    let xss_prefix = xss.clone().take(20).collect_vec();
    assert_eq!(
        xss_prefix
            .iter()
            .map(Vec::as_slice)
            .collect_vec()
            .as_slice(),
        out
    );
    assert_eq!(xss.count(), out_len);
}

#[test]
fn test_shortlex_unique_vecs_min_length() {
    shortlex_unique_vecs_min_length_small_helper(0, nevers(), 1, &[&[]]);
    shortlex_unique_vecs_min_length_small_helper(4, nevers(), 0, &[]);
    shortlex_unique_vecs_min_length_small_helper(0, exhaustive_units(), 2, &[&[], &[()]]);
    shortlex_unique_vecs_min_length_small_helper(5, exhaustive_units(), 0, &[]);
    shortlex_unique_vecs_min_length_small_helper(
        0,
        exhaustive_bools(),
        5,
        &[&[], &[false], &[true], &[false, true], &[true, false]],
    );
    shortlex_unique_vecs_min_length_small_helper(
        1,
        exhaustive_bools(),
        4,
        &[&[false], &[true], &[false, true], &[true, false]],
    );
    shortlex_unique_vecs_min_length_small_helper(
        0,
        'a'..='c',
        16,
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['b', 'c'],
            &['c', 'a'],
            &['c', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'b'],
            &['b', 'a', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    shortlex_unique_vecs_min_length_small_helper(
        2,
        'a'..='c',
        12,
        &[
            &['a', 'b'],
            &['a', 'c'],
            &['b', 'a'],
            &['b', 'c'],
            &['c', 'a'],
            &['c', 'b'],
            &['a', 'b', 'c'],
            &['a', 'c', 'b'],
            &['b', 'a', 'c'],
            &['b', 'c', 'a'],
            &['c', 'a', 'b'],
            &['c', 'b', 'a'],
        ],
    );
    shortlex_unique_vecs_min_length_helper(
        0,
        exhaustive_ascii_chars(),
        &[
            &[],
            &['a'],
            &['b'],
            &['c'],
            &['d'],
            &['e'],
            &['f'],
            &['g'],
            &['h'],
            &['i'],
            &['j'],
            &['k'],
            &['l'],
            &['m'],
            &['n'],
            &['o'],
            &['p'],
            &['q'],
            &['r'],
            &['s'],
        ],
    );
    shortlex_unique_vecs_min_length_helper(
        3,
        exhaustive_ascii_chars(),
        &[
            &['a', 'b', 'c'],
            &['a', 'b', 'd'],
            &['a', 'b', 'e'],
            &['a', 'b', 'f'],
            &['a', 'b', 'g'],
            &['a', 'b', 'h'],
            &['a', 'b', 'i'],
            &['a', 'b', 'j'],
            &['a', 'b', 'k'],
            &['a', 'b', 'l'],
            &['a', 'b', 'm'],
            &['a', 'b', 'n'],
            &['a', 'b', 'o'],
            &['a', 'b', 'p'],
            &['a', 'b', 'q'],
            &['a', 'b', 'r'],
            &['a', 'b', 's'],
            &['a', 'b', 't'],
            &['a', 'b', 'u'],
            &['a', 'b', 'v'],
        ],
    );
}
