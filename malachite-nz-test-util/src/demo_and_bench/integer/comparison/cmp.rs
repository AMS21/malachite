use crate::bench::bucketers::triple_3_pair_integer_max_bit_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::runner::Runner;
use malachite_nz_test_util::generators::{integer_pair_gen, integer_pair_gen_nrm};
use std::cmp::Ordering;

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_integer_cmp);
    register_bench!(runner, benchmark_integer_cmp_library_comparison);
}

fn demo_integer_cmp(gm: GenMode, config: GenConfig, limit: usize) {
    for (x, y) in integer_pair_gen().get(gm, &config).take(limit) {
        match x.cmp(&y) {
            Ordering::Less => println!("{} < {}", x, y),
            Ordering::Equal => println!("{} = {}", x, y),
            Ordering::Greater => println!("{} > {}", x, y),
        }
    }
}

#[allow(unused_must_use)]
fn benchmark_integer_cmp_library_comparison(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Integer.cmp(&Integer)",
        BenchmarkType::LibraryComparison,
        integer_pair_gen_nrm().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_integer_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (x, y))| no_out!(x.cmp(&y))),
            ("num", &mut |((x, y), _, _)| no_out!(x.cmp(&y))),
            ("rug", &mut |(_, (x, y), _)| no_out!(x.cmp(&y))),
        ],
    );
}