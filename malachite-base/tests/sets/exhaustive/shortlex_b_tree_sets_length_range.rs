use malachite_base::bools::exhaustive::exhaustive_bools;
use malachite_base::nevers::nevers;
use malachite_base::sets::exhaustive::shortlex_b_tree_sets_length_range;
use malachite_base::test_util::sets::exhaustive::exhaustive_b_tree_sets_small_helper_helper;
use malachite_base::tuples::exhaustive::exhaustive_units;
use std::collections::BTreeSet;
use std::fmt::Debug;

fn shortlex_b_tree_sets_length_range_small_helper<I: Clone + Iterator>(
    a: u64,
    b: u64,
    xs: I,
    out_len: usize,
    out: &[BTreeSet<I::Item>],
) where
    I::Item: Clone + Debug + Eq + Ord,
{
    exhaustive_b_tree_sets_small_helper_helper(
        shortlex_b_tree_sets_length_range(a, b, xs),
        out_len,
        out,
    );
}

#[test]
fn test_shortlex_b_tree_sets_length_range() {
    shortlex_b_tree_sets_length_range_small_helper(0, 5, nevers(), 1, &[btreeset! {}]);
    shortlex_b_tree_sets_length_range_small_helper(6, 10, nevers(), 0, &[]);
    shortlex_b_tree_sets_length_range_small_helper(
        0,
        5,
        exhaustive_units(),
        2,
        &[btreeset! {}, btreeset! {()}],
    );
    shortlex_b_tree_sets_length_range_small_helper(1, 0, exhaustive_bools(), 0, &[]);
    shortlex_b_tree_sets_length_range_small_helper(1, 1, exhaustive_bools(), 0, &[]);
    shortlex_b_tree_sets_length_range_small_helper(
        0,
        2,
        exhaustive_bools(),
        3,
        &[btreeset! {}, btreeset! {false}, btreeset! {true}],
    );
    shortlex_b_tree_sets_length_range_small_helper(
        2,
        4,
        exhaustive_bools(),
        1,
        &[btreeset! {false, true}],
    );
    shortlex_b_tree_sets_length_range_small_helper(
        1,
        2,
        'a'..='c',
        3,
        &[btreeset! {'a'}, btreeset! {'b'}, btreeset! {'c'}],
    );
}
