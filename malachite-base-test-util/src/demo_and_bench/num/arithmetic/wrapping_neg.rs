use malachite_base::num::basic::signeds::PrimitiveSigned;
use malachite_base::num::basic::unsigneds::PrimitiveUnsigned;
use malachite_base_test_util::bench::bucketers::{signed_bit_bucketer, unsigned_bit_bucketer};
use malachite_base_test_util::bench::{run_benchmark, BenchmarkType};
use malachite_base_test_util::generators::common::{GenConfig, GenMode};
use malachite_base_test_util::generators::{signed_gen, unsigned_gen};
use malachite_base_test_util::runner::Runner;

pub(crate) fn register(runner: &mut Runner) {
    register_unsigned_demos!(runner, demo_wrapping_neg_assign_unsigned);
    register_signed_demos!(runner, demo_wrapping_neg_assign_signed);
    register_unsigned_benches!(runner, benchmark_wrapping_neg_assign_unsigned);
    register_signed_benches!(runner, benchmark_wrapping_neg_assign_signed);
}

fn demo_wrapping_neg_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for mut u in unsigned_gen::<T>().get(gm, &config).take(limit) {
        let old_u = u;
        u.wrapping_neg_assign();
        println!("u := {}; u.wrapping_neg_assign(); u = {}", old_u, u);
    }
}

fn demo_wrapping_neg_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
) {
    for mut i in signed_gen::<T>().get(gm, &config).take(limit) {
        let old_i = i;
        i.wrapping_neg_assign();
        println!("i := {}; i.wrapping_neg_assign(); i = {}", old_i, i);
    }
}

fn benchmark_wrapping_neg_assign_unsigned<T: PrimitiveUnsigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_neg_assign()", T::NAME),
        BenchmarkType::Single,
        unsigned_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &unsigned_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.wrapping_neg_assign())],
    );
}

fn benchmark_wrapping_neg_assign_signed<T: PrimitiveSigned>(
    gm: GenMode,
    config: GenConfig,
    limit: usize,
    file_name: &str,
) {
    run_benchmark(
        &format!("{}.wrapping_neg_assign()", T::NAME),
        BenchmarkType::Single,
        signed_gen::<T>().get(gm, &config),
        gm.name(),
        limit,
        file_name,
        &signed_bit_bucketer(),
        &mut [("Malachite", &mut |mut i| i.wrapping_neg_assign())],
    );
}