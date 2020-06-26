use std::cmp::Ordering;

use malachite_base::orderings::exhaustive::{
    exhaustive_orderings, exhaustive_orderings_increasing,
};

#[test]
fn test_exhaustive_orderings_increasing() {
    assert_eq!(
        exhaustive_orderings_increasing().collect::<Vec<Ordering>>(),
        &[Ordering::Less, Ordering::Equal, Ordering::Greater]
    );
}

#[test]
fn test_exhaustive_orderings() {
    assert_eq!(
        exhaustive_orderings().collect::<Vec<Ordering>>(),
        &[Ordering::Equal, Ordering::Less, Ordering::Greater]
    );
}