// Copyright © 2024 Mikhail Hogrefe
//
// This file is part of Malachite.
//
// Malachite is free software: you can redistribute it and/or modify it under the terms of the GNU
// Lesser General Public License (LGPL) as published by the Free Software Foundation; either version
// 3 of the License, or (at your option) any later version. See <https://www.gnu.org/licenses/>.

use crate::test_util::common::rug_float_significant_bits;
use core::cmp::Ordering;
use malachite_base::num::conversion::traits::ExactFrom;
use rug::float::Round;
use rug::ops::AssignRound;

pub fn rug_square_prec_round(x: &rug::Float, prec: u64, rm: Round) -> (rug::Float, Ordering) {
    let mut sum = rug::Float::with_val(u32::exact_from(prec), 0);
    let o = sum.assign_round(x.square_ref(), rm);
    (sum, o)
}

pub fn rug_square_prec(x: &rug::Float, prec: u64) -> (rug::Float, Ordering) {
    rug_square_prec_round(x, prec, Round::Nearest)
}

pub fn rug_square_round(x: &rug::Float, rm: Round) -> (rug::Float, Ordering) {
    rug_square_prec_round(x, rug_float_significant_bits(x), rm)
}

pub fn rug_square(x: &rug::Float) -> rug::Float {
    rug_square_prec_round(x, rug_float_significant_bits(x), Round::Nearest).0
}