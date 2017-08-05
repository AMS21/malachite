use common::LARGE_LIMIT;
use malachite_native::integer as native;
use malachite_native::traits::NotAssign;
use malachite_gmp::integer as gmp;
use malachite_test::common::gmp_integer_to_native;
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::integers_geometric::natural_u32s_geometric;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{log_pairs, random_pairs};
use std::str::FromStr;

#[test]
fn test_set_bit() {
    let test = |u, index, out| {
        let mut n = native::Integer::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());

        let mut n = gmp::Integer::from_str(u).unwrap();
        n.set_bit(index);
        assert_eq!(n.to_string(), out);
        assert!(n.is_valid());
    };
    test("0", 10, "1024");
    test("100", 0, "101");
    test("1000000000000", 10, "1000000001024");
    test("1000000000000", 100, "1267650600228229402496703205376");
    test("5", 100, "1267650600228229401496703205381");
    test("-1", 5, "-1");
    test("-1", 100, "-1");
    test("-33", 5, "-1");
    test("-1267650600228229401496703205377", 100, "-1");
    test("-32", 0, "-31");
    test("-1000000000000", 10, "-999999998976");
    test("-1000000000000", 100, "-1000000000000");
    test("-1267650600228229402496703205376", 100, "-1000000000000");
    test("-18446744078004518912", 0, "-18446744078004518911");
    test("-18446744078004518912", 32, "-18446744078004518912");
    test("-18446744078004518912", 33, "-18446744078004518912");
    test("-18446744078004518912", 64, "-4294967296");
    test("-18446744078004518912", 65, "-18446744078004518912");
}

#[test]
fn set_bit_properties() {
    // n.set_bit(index) is equivalent for malachite-gmp and malachite-native.
    // TODO n.set_bit(index) is equivalent to n.assign_bit(index, true).
    // n.set_bit(index); n != 0
    // Setting a bit does not decrease n.
    // If n.get_bit(index), setting at index won't do anything.
    // If !n.get_bit(index), setting and then clearing at index won't do anything.
    // { n.set_bit(index) } is equivalent to { n := !n; n.clear_bit(index); n := !n }
    let integer_and_u64 = |mut gmp_n: gmp::Integer, index: u64| {
        let mut n = gmp_integer_to_native(&gmp_n);
        let old_n = n.clone();
        gmp_n.set_bit(index);
        assert!(gmp_n.is_valid(), "Invalid! {} {}", n, index);

        n.set_bit(index);
        assert!(n.is_valid());
        assert_eq!(gmp_integer_to_native(&gmp_n), n);

        //TODO
        /*
        let mut n2 = old_n.clone();
        n2.assign_bit(index, true);
        assert_eq!(n2, n);
        */


        assert_ne!(n, 0);
        assert!(n >= old_n);
        if old_n.get_bit(index) {
            assert_eq!(n, old_n);
        } else {
            assert_ne!(n, old_n);
            n.clear_bit(index);
            assert_eq!(n, old_n);
        }

        let mut m = !&old_n;
        m.clear_bit(index);
        m.not_assign();
        let mut n = old_n.clone();
        n.set_bit(index);
        assert_eq!(m, n);
    };

    for (n, index) in log_pairs(exhaustive_integers(), exhaustive_u::<u64>()).take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }

    for (n, index) in random_pairs(&EXAMPLE_SEED,
                                   &(|seed| random_integers(seed, 32)),
                                   &(|seed| natural_u32s_geometric(seed, 32).map(|i| i as u64)))
                .take(LARGE_LIMIT) {
        integer_and_u64(n, index);
    }
}
