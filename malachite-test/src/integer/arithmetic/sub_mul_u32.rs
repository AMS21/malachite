use common::gmp_integer_to_native;
use malachite_native::integer as native;
use malachite_native::traits::SubMul as native_sub_mul;
use malachite_native::traits::SubMulAssign as native_sub_mul_assign;
use malachite_gmp::integer as gmp;
use malachite_gmp::traits::SubMul as gmp_sub_mul;
use malachite_gmp::traits::SubMulAssign as gmp_sub_mul_assign;
use rust_wheels::benchmarks::{benchmark_2, BenchmarkOptions2, benchmark_4, BenchmarkOptions4};
use rust_wheels::iterators::common::EXAMPLE_SEED;
use rust_wheels::iterators::general::random_x;
use rust_wheels::iterators::integers::{exhaustive_integers, random_integers};
use rust_wheels::iterators::primitive_ints::exhaustive_u;
use rust_wheels::iterators::tuples::{exhaustive_triples, random_triples};
use std::cmp::max;

pub fn demo_exhaustive_integer_sub_mul_assign_u32(limit: usize) {
    for (mut a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_random_integer_sub_mul_assign_u32(limit: usize) {
    for (mut a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        a.sub_mul_assign(b, c);
        println!(
            "a := {}; x.sub_mul_assign({}, {}); x = {}",
            a_old,
            b_old,
            c,
            a
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_assign_u32_ref(limit: usize) {
    for (mut a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_random_integer_sub_mul_assign_u32_ref(limit: usize) {
    for (mut a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        a.sub_mul_assign(&b, c);
        println!("a := {}; x.sub_mul_assign(&{}, {}); x = {}", a_old, b, c, a);
    }
}

pub fn demo_exhaustive_integer_sub_mul_u32(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

pub fn demo_random_integer_sub_mul_u32(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!("{}.sub_mul({}, {}) = {}", a_old, b_old, c, a.sub_mul(b, c));
    }
}

pub fn demo_exhaustive_integer_sub_mul_u32_val_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_random_integer_sub_mul_u32_val_ref(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        println!("{}.sub_mul(&{}, {}) = {}", a_old, b, c, a.sub_mul(&b, c));
    }
}

pub fn demo_exhaustive_integer_sub_mul_u32_ref_val(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

pub fn demo_random_integer_sub_mul_u32_ref_val(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        let b_old = b.clone();
        println!(
            "(&{}).sub_mul({}, {}) = {}",
            a_old,
            b_old,
            c,
            (&a).sub_mul(b, c)
        );
    }
}

pub fn demo_exhaustive_integer_sub_mul_u32_ref_ref(limit: usize) {
    for (a, b, c) in exhaustive_triples(
        exhaustive_integers(),
        exhaustive_integers(),
        exhaustive_u::<u32>(),
    ).take(limit)
    {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn demo_random_integer_sub_mul_u32_ref_ref(limit: usize) {
    for (a, b, c) in random_triples(
        &EXAMPLE_SEED,
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_integers(seed, 32)),
        &(|seed| random_x(seed)),
    ).take(limit)
    {
        let a_old = a.clone();
        println!(
            "(&{}).sub_mul(&{}, {}) = {}",
            a_old,
            b,
            c,
            (&a).sub_mul(&b, c)
        );
    }
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul_assign(b, c)),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_u32_evaluation_strategy(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, u32) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(&b, c)
                      }),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_u32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, u32) evaluation strategy");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(&b, c)
                      }),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_u32_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer -= Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_u32_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(Integer, u32)",
        g_name: "Integer -= Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_assign_u32_ref_algorithms(
    limit: usize,
    file_name: &str,
) {
    println!("benchmarking exhaustive Integer.sub_mul_assign(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        g_name: "Integer -= \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_assign_u32_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul_assign(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| {
                          a.sub_mul_assign(&b, c)
                      }),
        function_g: &(|(mut a, b, c): (native::Integer, native::Integer, u32)| a -= &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32)",
        g_name: "Integer -= \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul\\\\_assign(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul(Integer, u32)");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (gmp::Integer, gmp::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        x_cons: &(|t| t.clone()),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "malachite-gmp",
        g_name: "malachite-native",
        title: "Integer.sub\\\\_mul(Integer, u32)",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32_evaluation_strategy(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, u32) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_h: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_i: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        z_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        w_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        h_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        i_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32_evaluation_strategy(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(Integer, u32) evaluation strategy");
    benchmark_4(BenchmarkOptions4 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_h: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_i: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        z_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        w_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        h_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        i_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) evaluation strategy",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer - Integer * u32",
        title: "Integer.sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32_algorithms(limit: usize, scale: u32, file_name: &str) {
    println!("benchmarking random Integer.sub_mul(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(Integer, u32)",
        g_name: "Integer - Integer * u32",
        title: "Integer.sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32_val_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive Integer.sub_mul(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "Integer - \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32_val_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random Integer.sub_mul(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| a.sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "Integer.sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "Integer - \\\\&Integer * u32",
        title: "Integer.sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32_ref_val_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Integer).sub_mul(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        g_name: "(\\\\&Integer) - Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32_ref_val_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random (&Integer).sub_mul(Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(Integer, u32)",
        g_name: "(\\\\&Integer) - Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_exhaustive_integer_sub_mul_u32_ref_ref_algorithms(limit: usize, file_name: &str) {
    println!("benchmarking exhaustive (&Integer).sub_mul(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: exhaustive_triples(
            exhaustive_integers(),
            exhaustive_integers(),
            exhaustive_u::<u32>(),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "(\\\\&Integer) - \\\\&Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_random_integer_sub_mul_u32_ref_ref_algorithms(
    limit: usize,
    scale: u32,
    file_name: &str,
) {
    println!("benchmarking random (&Integer).sub_mul(&Integer, u32) algorithms");
    benchmark_2(BenchmarkOptions2 {
        xs: random_triples(
            &EXAMPLE_SEED,
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_integers(seed, scale)),
            &(|seed| random_x(seed)),
        ),
        function_f: &(|(a, b, c): (native::Integer, native::Integer, u32)| (&a).sub_mul(&b, c)),
        function_g: &(|(a, b, c): (native::Integer, native::Integer, u32)| &a - &b * c),
        x_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        y_cons: &(|&(ref a, ref b, c)| (gmp_integer_to_native(a), gmp_integer_to_native(b), c)),
        x_param: &(|&(ref a, ref b, _)| max(a.significant_bits(), b.significant_bits()) as usize),
        limit: limit,
        f_name: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32)",
        g_name: "(\\\\&Integer) - \\\\&Integer * u32",
        title: "(\\\\&Integer).sub\\\\_mul(\\\\&Integer, u32) algorithms",
        x_axis_label: "max(a.significant\\\\_bits(), b.significant\\\\_bits())",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}