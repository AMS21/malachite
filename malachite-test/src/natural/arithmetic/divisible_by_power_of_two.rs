use common::{m_run_benchmark, BenchmarkType, DemoBenchRegistry, GenerationMode, ScaleType};
use inputs::natural::pairs_of_natural_and_small_unsigned;
use malachite_base::num::SignificantBits;

pub(crate) fn register(registry: &mut DemoBenchRegistry) {
    register_demo!(registry, demo_natural_divisible_by_power_of_two);
    register_bench!(
        registry,
        Large,
        benchmark_natural_divisible_by_power_of_two_algorithms
    );
}

fn demo_natural_divisible_by_power_of_two(gm: GenerationMode, limit: usize) {
    for (n, pow) in pairs_of_natural_and_small_unsigned(gm).take(limit) {
        if n.divisible_by_power_of_two(pow) {
            println!("{} is divisible by 2^{}", n, pow);
        } else {
            println!("{} is not divisible by 2^{}", n, pow);
        }
    }
}

fn benchmark_natural_divisible_by_power_of_two_algorithms(
    gm: GenerationMode,
    limit: usize,
    file_name: &str,
) {
    m_run_benchmark(
        "Natural.divisible_by_power_of_two(u32)",
        BenchmarkType::Algorithms,
        pairs_of_natural_and_small_unsigned(gm),
        gm.name(),
        limit,
        file_name,
        &(|&(ref n, _)| n.significant_bits() as usize),
        "n.significant_bits()",
        &mut [
            (
                "Natural.divisible_by_power_of_2(u32)",
                &mut (|(n, pow)| no_out!(n.divisible_by_power_of_two(pow))),
            ),
            (
                "Natural.trailing_zeros().map_or(true, |z| z >= u32)",
                &mut (|(n, pow)| no_out!(n.trailing_zeros().map_or(true, |z| z >= u64::from(pow)))),
            ),
        ],
    );
}
