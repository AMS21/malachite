use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::integer::{nm_pairs_of_integer_and_unsigned, pairs_of_integer_and_unsigned};
use malachite_base::num::Assign;
use malachite_base::num::SignificantBits;
use num::BigInt;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_integer_assign_u64);
    register_bench!(
        registry,
        Large,
        benchmark_integer_assign_u64_library_comparison
    );
}

pub fn num_assign_u64(x: &mut BigInt, u: u64) {
    *x = BigInt::from(u);
}

fn demo_integer_assign_u64(gm: GenerationMode, limit: usize) {
    for (mut n, u) in pairs_of_integer_and_unsigned::<u64>(gm).take(limit) {
        let n_old = n.clone();
        n.assign(u);
        println!("x := {}; x.assign({}); x = {}", n_old, u, n);
    }
}

fn benchmark_integer_assign_u64_library_comparison(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Integer.assign(u64)",
        BenchmarkType::LibraryComparison,
        nm_pairs_of_integer_and_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(_, (ref n, _))| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            ("malachite", &mut (|(_, (mut x, y))| x.assign(y))),
            ("num", &mut (|((mut x, y), _)| num_assign_u64(&mut x, y))),
        ],
    );
}
