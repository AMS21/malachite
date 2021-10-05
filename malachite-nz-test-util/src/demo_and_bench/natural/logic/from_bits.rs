use malachite_base::num::logic::traits::BitConvertible;
use malachite_base_test_util::bench::bucketers::vec_len_bucketer;
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::bool_vec_gen;
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::num::logic::bit_convertible::{
    from_bits_asc_alt, from_bits_desc_alt,
};
use malachite_base_test_util::runner::Runner;
use malachite_nz::natural::Natural;
use malachite_nz_test_util::natural::logic::from_bits::{
    from_bits_asc_naive, from_bits_desc_naive,
};

pub(crate) fn register(runner: &mut Runner) {
    register_demo!(runner, demo_natural_from_bits_asc);
    register_demo!(runner, demo_natural_from_bits_desc);

    register_bench!(runner, benchmark_natural_from_bits_asc_algorithms);
    register_bench!(runner, benchmark_natural_from_bits_desc_algorithms);
}

fn demo_natural_from_bits_asc(gm: GenMode, config: GenConfig, limit: usize) {
    for bits in bool_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_bits_asc({:?}) = {:?}",
            bits,
            Natural::from_bits_asc(bits.iter().cloned())
        );
    }
}

fn demo_natural_from_bits_desc(gm: GenMode, config: GenConfig, limit: usize) {
    for bits in bool_vec_gen().get(gm, &config).take(limit) {
        println!(
            "from_bits_desc({:?}) = {:?}",
            bits,
            Natural::from_bits_desc(bits.iter().cloned())
        );
    }
}

fn benchmark_natural_from_bits_asc_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_bits_asc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        bool_vec_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |bits| {
                no_out!(Natural::from_bits_asc(bits.into_iter()))
            }),
            ("alt", &mut |bits| {
                no_out!(from_bits_asc_alt::<Natural, _>(bits.into_iter()))
            }),
            ("naive", &mut |bits| {
                no_out!(from_bits_asc_naive(bits.into_iter()))
            }),
        ],
    );
}

fn benchmark_natural_from_bits_desc_algorithms(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        "Natural::from_bits_desc<I: Iterator<Item=bool>>(I)",
        BenchmarkType::Algorithms,
        bool_vec_gen().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &vec_len_bucketer(),
        &mut [
            ("default", &mut |ref bits| {
                no_out!(Natural::from_bits_desc(bits.iter().cloned()))
            }),
            ("alt", &mut |ref bits| {
                no_out!(from_bits_desc_alt::<Natural, _>(bits.iter().cloned()))
            }),
            ("naive", &mut |ref bits| {
                no_out!(from_bits_desc_naive(bits.iter().cloned()))
            }),
        ],
    );
}