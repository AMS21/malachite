use malachite_base::limbs::limbs_test_zero;
use malachite_base::misc::Max;
use malachite_base::num::{Parity, PrimitiveInteger};
use natural::arithmetic::add::{
    limbs_add_same_length_to_out, limbs_add_to_out, limbs_slice_add_same_length_in_place_left,
};
use natural::arithmetic::add_limb::limbs_slice_add_limb_in_place;
use natural::arithmetic::mul::fft::{_limbs_mul_fft, _limbs_mul_fft_best_k, mpn_fft_next_size};
use natural::arithmetic::mul::{limbs_mul_greater_to_out, limbs_mul_same_length_to_out};
use natural::arithmetic::shr_u::limbs_slice_shr_in_place;
use natural::arithmetic::sub::limbs_sub_same_length_in_place_left;
use natural::arithmetic::sub::{
    _limbs_sub_same_length_with_borrow_in_place_right, limbs_sub_in_place_left,
    limbs_sub_same_length_to_out, limbs_sub_to_out,
};
use natural::arithmetic::sub_limb::limbs_sub_limb_in_place;
use platform::Limb;
use std::cmp::min;

//TODO tune
pub(crate) const MULMOD_BNM1_THRESHOLD: usize = 13;
pub(crate) const MUL_FFT_MODF_THRESHOLD: usize = 396;

// This is mpn_mulmod_bnm1_next_size from mpn/generic/mulmod_bnm1.c.
pub(crate) fn _limbs_mul_mod_limb_width_to_n_minus_1_next_size(n: usize) -> usize {
    if n < MULMOD_BNM1_THRESHOLD {
        n
    } else if n <= (MULMOD_BNM1_THRESHOLD - 1) << 2 {
        (n + 1) >> 1 << 1
    } else if n <= (MULMOD_BNM1_THRESHOLD - 1) << 3 {
        (n + 3) >> 2 << 2
    } else {
        let ceiling_half_n = (n + 1) >> 1;
        if ceiling_half_n < MUL_FFT_MODF_THRESHOLD {
            (n + 7) >> 3 << 3
        } else {
            mpn_fft_next_size(ceiling_half_n, _limbs_mul_fft_best_k(ceiling_half_n, false)) << 1
        }
    }
}

// This is mpn_mulmod_bnm1_itch from gmp-impl.h.
pub(crate) fn _limbs_mul_mod_limb_width_to_n_minus_1_scratch_size(
    n: usize,
    xs_len: usize,
    ys_len: usize,
) -> usize {
    let half_n = n >> 1;
    if xs_len > half_n {
        if ys_len > half_n {
            2 * n + 4
        } else {
            n + 4 + half_n
        }
    } else {
        n + 4
    }
}

/// Interpreting two equal-length, nonempty slices of `Limb`s as the limbs (in ascending order) of
/// two `Natural`s, multiplies the `Natural`s mod `Limb::WIDTH`<sup>n</sup> - 1, where n is the
/// length of either slice. The result is semi-normalized: zero is represented as either 0 or
/// `Limb::WIDTH`<sup>n</sup> - 1. The limbs of the result are written to `out`. `out` should have
/// length at least n, and `scratch` at least 2 * n. This is the basecase algorithm.
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// # Panics
/// Panics if `xs` and `ys` have different lengths, if `out` or `scratch` are too short, or if the
/// input slices are empty.
///
/// This is mpn_bc_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c.
fn _limbs_mul_mod_limb_width_to_n_minus_1_basecase(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let n = xs.len();
    assert_ne!(n, 0);
    limbs_mul_same_length_to_out(scratch, xs, ys);
    let (scratch_lo, scratch_hi) = scratch.split_at(n);
    if limbs_add_same_length_to_out(out, scratch_lo, &scratch_hi[..n]) {
        // If cy == 1, then the value of out is at most B ^ n - 2, so there can be no overflow when
        // adding in the carry.
        limbs_slice_add_limb_in_place(&mut out[..n], 1);
    }
}

/// Interpreting the first n + 1 limbs of two slices of `Limb`s as the limbs (in ascending order) of
/// two `Natural`s, multiplies the `Natural`s mod `Limb::WIDTH`<sup>n</sup> + 1. The limbs of the
/// result are written to `out`, which should have length at least 2 * n + 2.
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// # Panics
/// Panics if `xs`, `ys`, or `out` are too short, or if n is zero.
///
// This is mpn_bc_mulmod_bnp1 from mpn/generic/mulmod_bnm1.c, where rp == tp.
fn _limbs_mul_mod_limb_width_to_n_plus_1_basecase(
    out: &mut [Limb],
    xs: &[Limb],
    ys: &[Limb],
    n: usize,
) {
    assert_ne!(0, n);
    limbs_mul_same_length_to_out(out, &xs[..n + 1], &ys[..n + 1]);
    assert_eq!(out[2 * n + 1], 0);
    let mut carry = out[2 * n];
    assert_ne!(carry, Limb::MAX);
    {
        let (out_lo, out_hi) = out.split_at_mut(n);
        if limbs_sub_same_length_in_place_left(out_lo, &out_hi[..n]) {
            carry += 1;
        }
    }
    out[n] = 0;
    assert!(!limbs_slice_add_limb_in_place(&mut out[..n + 1], carry));
}

// First k to use for an FFT mod-F multiply. A mod-F FFT is an order log(2 ^ k) / log(2 ^ (k - 1))
// algorithm, so k = 3 is merely 1.5 like Karatsuba, whereas k = 4 is 1.33 which is faster than
// Toom3 at 1.485.
const FFT_FIRST_K: usize = 4;

/// Interpreting two nonempty slices of `Limb`s as the limbs (in ascending order) of two `Natural`s,
/// multiplies the `Natural`s mod `Limb::WIDTH`<sup>n</sup> - 1. The limbs of the result are written
/// to `out`.
///
/// The result is expected to be 0 if and only if one of the operands already is. Otherwise the
/// class 0 mod (`Limb::WIDTH`<sup>n</sup> - 1) is represented by `Limb::WIDTH`<sup>n</sup> - 1.
/// This should not be a problem if `_limbs_mul_mod_limb_width_to_n_minus_1` is used to combine
/// results and obtain a natural number when one knows in advance that the final value is less than
/// `Limb::WIDTH`<sup>n</sup> - 1. Moreover it should not be a problem if
/// `_limbs_mul_mod_limb_width_to_n_minus_1` is used to compute the full product with `xs.len()` +
/// `ys.len()` <= n, because this condition implies
/// (`Limb::WIDTH`<sup>`xs.len()`</sup> - 1)(`Limb::WIDTH`<sup>`ys.len()`</sup> - 1) <
/// `Limb::WIDTH`<sup>n</sup> - 1.
///
/// Requires 0 < `ys.len()` <= `xs.len()` <= n and an + `ys.len()` > n / 2.
/// Scratch need: n + (need for recursive call OR n + 4). This gives
/// S(n) <= n + MAX (n + 4, S(n / 2)) <= 2 * n + 4
///
/// Time: TODO
///
/// Additional memory: TODO
///
/// # Panics
/// Panics if `xs` is shorter than `ys`, if `ys` is empty, is `xs` is longer than n, or if `out` or
/// `scratch` are too short.
///
/// This is mpn_mulmod_bnm1 from mpn/generic/mulmod_bnm1.c.
pub fn _limbs_mul_mod_limb_width_to_n_minus_1(
    out: &mut [Limb],
    n: usize,
    xs: &[Limb],
    ys: &[Limb],
    scratch: &mut [Limb],
) {
    let xs_len = xs.len();
    let ys_len = ys.len();
    assert_ne!(0, ys_len);
    assert!(ys_len <= xs_len);
    assert!(xs_len <= n);

    if n < MULMOD_BNM1_THRESHOLD || n.odd() {
        if ys_len < n {
            if xs_len + ys_len <= n {
                limbs_mul_greater_to_out(out, xs, ys);
            } else {
                limbs_mul_greater_to_out(scratch, xs, ys);
                if limbs_add_to_out(out, &scratch[..n], &scratch[n..xs_len + ys_len]) {
                    assert!(!limbs_slice_add_limb_in_place(&mut out[..n], 1));
                }
            }
        } else {
            _limbs_mul_mod_limb_width_to_n_minus_1_basecase(out, &xs[..n], ys, scratch);
        }
    } else {
        let half_n = n >> 1;

        // We need at least xs_len + ys_len >= half_n, to be able to fit one of the recursive
        // products at out. Requiring strict inequality makes the code slightly simpler. If desired,
        // we could avoid this restriction by initially halving n as long as n is even and
        // xs_len + ys_len <= n/2.
        assert!(xs_len + ys_len > half_n);

        // Compute xm = a * b mod (Limb::WIDTH ^ half_n - 1),
        // xp = a * b mod (Limb::WIDTH ^ half_n + 1) and Chinese-Remainder-Theorem together as
        //
        // x = -xp * Limb::WIDTH ^ half_n + (Limb::WIDTH ^ half_n + 1) *
        // ((xp + xm) / 2 mod (Limb::WIDTH ^ half_n - 1))
        if xs_len > half_n {
            let (xs_0, xs_1) = xs.split_at(half_n);
            let carry = limbs_add_to_out(scratch, xs_0, xs_1);
            let (scratch_0, scratch_hi) = scratch.split_at_mut(half_n);
            if carry {
                assert!(!limbs_slice_add_limb_in_place(scratch_0, 1));
            }
            if ys_len > half_n {
                let (ys_0, ys_1) = ys.split_at(half_n);
                let carry = limbs_add_to_out(scratch_hi, ys_0, ys_1);
                let (scratch_1, scratch_2) = scratch_hi.split_at_mut(half_n);
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_1, 1));
                }
                _limbs_mul_mod_limb_width_to_n_minus_1(
                    out, half_n, scratch_0, scratch_1, scratch_2,
                );
            } else {
                _limbs_mul_mod_limb_width_to_n_minus_1(out, half_n, scratch_0, ys, scratch_hi);
            }
        } else {
            _limbs_mul_mod_limb_width_to_n_minus_1(out, half_n, xs, ys, scratch);
        }
        {
            let mut anp = xs_len;
            let ap1_is_xs_0 = xs_len <= half_n;
            let mut bnp = ys_len;
            let bp1_is_ys_0 = ys_len <= half_n;
            let m = half_n + 1;
            if !ap1_is_xs_0 {
                let (xs_0, xs_1) = xs.split_at(half_n);
                let (scratch_2, scratch_3) = scratch[2 * m..].split_at_mut(m);
                let carry = limbs_sub_to_out(scratch_2, xs_0, xs_1);
                *scratch_2.last_mut().unwrap() = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(scratch_2, 1));
                }
                anp = half_n + *scratch_2.last_mut().unwrap() as usize;

                if !bp1_is_ys_0 {
                    let (ys_0, ys_1) = ys.split_at(half_n);
                    let carry = limbs_sub_to_out(scratch_3, ys_0, ys_1);
                    *scratch_3.last_mut().unwrap() = 0;
                    if carry {
                        assert!(!limbs_slice_add_limb_in_place(scratch_3, 1));
                    }
                    bnp = half_n + *scratch_3.last_mut().unwrap() as usize;
                }
            }

            let k = if half_n < MUL_FFT_MODF_THRESHOLD {
                0
            } else {
                min(
                    _limbs_mul_fft_best_k(half_n, false),
                    half_n.trailing_zeros() as usize,
                )
            };
            if k >= FFT_FIRST_K {
                if bp1_is_ys_0 {
                    if ap1_is_xs_0 {
                        scratch[half_n] = if _limbs_mul_fft(scratch, half_n, xs, ys, k) {
                            1
                        } else {
                            0
                        };
                    } else {
                        let (scratch_lo, scratch_hi) = scratch.split_at_mut(2 * m);
                        scratch_lo[half_n] =
                            if _limbs_mul_fft(scratch_lo, half_n, &scratch_hi[..anp], ys, k) {
                                1
                            } else {
                                0
                            };
                    }
                } else {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(2 * m);
                    scratch_lo[half_n] = if _limbs_mul_fft(
                        scratch_lo,
                        half_n,
                        if ap1_is_xs_0 { xs } else { &scratch_hi[..anp] },
                        &scratch_hi[m..bnp + m],
                        k,
                    ) {
                        1
                    } else {
                        0
                    };
                }
            } else if bp1_is_ys_0 {
                assert!(anp + bnp <= 2 * half_n + 1);
                assert!(anp + bnp > half_n);
                assert!(anp >= bnp);
                if ap1_is_xs_0 {
                    limbs_mul_greater_to_out(scratch, xs, ys);
                } else {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(2 * m);
                    limbs_mul_greater_to_out(scratch_lo, &scratch_hi[..anp], ys);
                }
                anp += bnp;
                anp -= half_n;
                assert!(anp <= half_n || scratch[2 * half_n] == 0);
                if anp > half_n {
                    anp -= 1;
                }
                let carry = {
                    let (scratch_lo, scratch_hi) = scratch.split_at_mut(half_n);
                    limbs_sub_in_place_left(scratch_lo, &scratch_hi[..anp])
                };
                scratch[half_n] = 0;
                if carry {
                    assert!(!limbs_slice_add_limb_in_place(&mut scratch[..m], 1));
                }
            } else {
                assert!(!ap1_is_xs_0);
                let (scratch_lo, scratch_hi) = scratch.split_at_mut(2 * m);
                _limbs_mul_mod_limb_width_to_n_plus_1_basecase(
                    scratch_lo,
                    scratch_hi,
                    &scratch_hi[m..],
                    half_n,
                );
            }
        }

        // Here the Chinese Remainder Theorem recomposition begins.
        //
        // let xm = (scratch + xm) / 2 = (scratch + xm) * Limb::WIDTH ^ half_n / 2 mod
        // (Limb::WIDTH ^ half_n - 1).
        // Division by 2 is a bitwise rotation.
        //
        // Assumes scratch normalised mod (Limb::WIDTH ^ half_n + 1).
        //
        // The residue class 0 is represented by [Limb::WIDTH ^ half_n - 1]; except when
        // both inputs are zero.
        //
        // scratch[half_n] == 1 implies limbs_test_zero(scratch[..half_n]).
        let mut carry = scratch[half_n];
        if limbs_slice_add_same_length_in_place_left(&mut out[..half_n], &scratch[..half_n]) {
            carry += 1;
        }
        if out[0].odd() {
            carry += 1;
        }
        limbs_slice_shr_in_place(&mut out[..half_n], 1);
        assert!(carry <= 2);
        let hi = carry << (Limb::WIDTH - 1); // carry.odd() << ...
        carry >>= 1;
        // We can have carry != 0 only if hi = 0...
        assert!(!out[half_n - 1].get_highest_bit());
        out[half_n - 1] |= hi;
        // ... out[half_n - 1] + carry can not overflow, the following increment is correct.
        // Next increment can not overflow; read the previous comments about carry.
        if carry != 0 {
            assert_eq!(carry, 1);
            assert!(!out[half_n - 1].get_highest_bit());
            assert!(!limbs_slice_add_limb_in_place(&mut out[..half_n], 1));
        }

        // Compute the highest half:
        // ([(scratch + xm) / 2 mod (Limb::WIDTH ^ half_n - 1)] - scratch) * Limb::WIDTH ^ half_n
        if xs_len + ys_len < n {
            let a = xs_len + ys_len - half_n;
            // Note that in this case, the only way the result can equal zero mod
            // Limb::WIDTH ^ n - 1 is if one of the inputs is zero, and then the output of both the
            // recursive calls and this CRT reconstruction is zero, not Limb::WIDTH ^ n - 1. Which
            // is good, since the latter representation doesn't fit in the output area.
            let bool_carry = {
                let (out_lo, out_hi) = out.split_at_mut(half_n);
                limbs_sub_same_length_to_out(out_hi, &out_lo[..a], &scratch[..a])
            };
            let mut carry = scratch[half_n];
            if _limbs_sub_same_length_with_borrow_in_place_right(
                &out[a..n - half_n],
                &mut scratch[a..n - half_n],
                bool_carry,
            ) {
                carry += 1;
            }
            assert!(xs_len + ys_len == n - 1 || limbs_test_zero(&scratch[a + 1..n - half_n]));
            if limbs_sub_limb_in_place(&mut out[..xs_len + ys_len], carry) {
                assert_eq!(scratch[a], 1);
            } else {
                assert_eq!(scratch[a], 0);
            }
        } else {
            let mut carry = scratch[half_n];
            {
                let (out_lo, out_hi) = out.split_at_mut(half_n);
                if limbs_sub_same_length_to_out(out_hi, out_lo, &scratch[..half_n]) {
                    carry += 1;
                }
            }
            // carry == 1 only if &scratch[..half_n + 1] is not zero, i.e. out[..half_n] is not
            // zero. The decrement will affect _at most_ the lowest half_n limbs.
            assert!(!limbs_sub_limb_in_place(&mut out[..2 * half_n], carry));
        }
    }
}