use common::{integer_to_rugint_integer, GenerationMode};
use inputs::integer::integers;
use malachite_base::num::SignificantBits;
use malachite_nz::integer::Integer;
use rugint;
use rust_wheels::benchmarks::{BenchmarkOptions2, benchmark_2};

pub fn demo_integer_to_u32(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u32({}) = {:?}", n, n.to_u32());
    }
}

pub fn demo_integer_to_u32_wrapping(gm: GenerationMode, limit: usize) {
    for n in integers(gm).take(limit) {
        println!("to_u32_wrapping({}) = {:?}", n, n.to_u32_wrapping());
    }
}

pub fn benchmark_integer_to_u32(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.to_u32()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.to_u32()),
        function_g: &(|n: rugint::Integer| n.to_u32()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer.to\\\\_u32()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}

pub fn benchmark_integer_to_u32_wrapping(gm: GenerationMode, limit: usize, file_name: &str) {
    println!("benchmarking {} Integer.to_u32_wrapping()", gm.name());
    benchmark_2(BenchmarkOptions2 {
        xs: integers(gm),
        function_f: &(|n: Integer| n.to_u32_wrapping()),
        function_g: &(|n: rugint::Integer| n.to_u32_wrapping()),
        x_cons: &(|x| x.clone()),
        y_cons: &(|x| integer_to_rugint_integer(x)),
        x_param: &(|n| n.significant_bits() as usize),
        limit,
        f_name: "malachite",
        g_name: "rugint",
        title: "Integer.to\\\\_u32\\\\_wrapping()",
        x_axis_label: "n.significant\\\\_bits()",
        y_axis_label: "time (ns)",
        file_name: &format!("benchmarks/{}", file_name),
    });
}
