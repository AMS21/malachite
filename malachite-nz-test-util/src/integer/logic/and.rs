use malachite_nz::integer::Integer;

use crate::integer::logic::{integer_op_bits, integer_op_limbs};

pub fn integer_and_alt_1(x: &Integer, y: &Integer) -> Integer {
    integer_op_bits(&|a, b| a && b, x, y)
}

pub fn integer_and_alt_2(x: &Integer, y: &Integer) -> Integer {
    integer_op_limbs(&|a, b| a & b, x, y)
}