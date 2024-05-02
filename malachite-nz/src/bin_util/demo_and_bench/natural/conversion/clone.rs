// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use malachite_base::test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base::test_util::generators::common::{GenConfig, GenMode};
use malachite_base::test_util::runner::Runner;
use malachite_nz::test_util::bench::bucketers::{
    triple_3_natural_bit_bucketer, triple_3_pair_natural_max_bit_bucketer,
};
use malachite_nz::test_util::generators::{
    natural_gen, natural_gen_nrm, natural_pair_gen, natural_pair_gen_nrm,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_clone);
    register_demo!(runner, demo_natural_clone_from);
    register_bench!(runner, benchmark_natural_clone_library_comparison);
    register_bench!(runner, benchmark_natural_clone_from_library_comparison);
}

fn demo_natural_clone(gm: GenMode, config: &GenConfig, limit: usize) {
    for n in natural_gen().get(gm, config).take(limit) {
        println!("clone({}) = {}", n, n.clone());
    }
}

fn demo_natural_clone_from(gm: GenMode, config: &GenConfig, limit: usize) {
    for (mut x, y) in natural_pair_gen().get(gm, config).take(limit) {
        let x_old = x.clone();
        x.clone_from(&y);
        println!("x := {x_old}; x.clone_from({y}); x = {x}");
    }
}

#[allow(clippy::redundant_clone, unused_must_use)]
fn benchmark_natural_clone_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.clone()",
        BenchmarkType::LibraryComparison,
        natural_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_natural_bit_bucketer("n"),
        &mut [
            ("Malachite", &mut |(_, _, n)| no_out!(n.clone())),
            ("num", &mut |(n, _, _)| no_out!(n.clone())),
            ("rug", &mut |(_, n, _)| no_out!(n.clone())),
        ],
    );
}

fn benchmark_natural_clone_from_library_comparison(
    gm: GenMode,
    config: &GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural.clone_from(&Natural)",
        BenchmarkType::LibraryComparison,
        natural_pair_gen_nrm().get(gm, config),
        gm.name(),
        limit,
        file_name,
        &triple_3_pair_natural_max_bit_bucketer("x", "y"),
        &mut [
            ("Malachite", &mut |(_, _, (mut x, y))| x.clone_from(&y)),
            ("num", &mut |((mut x, y), _, _)| x.clone_from(&y)),
            ("rug", &mut |(_, (mut x, y), _)| x.clone_from(&y)),
        ],
    );
}
