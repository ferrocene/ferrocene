use core::num::bignum::Big32x40 as Big;
use core::num::flt2dec::strategy::dragon::*;

use super::super::*;

#[test]
fn test_mul_pow10() {
    let mut prevpow10 = Big::from_small(1);
    for i in 1..340 {
        let mut curpow10 = Big::from_small(1);
        mul_pow10(&mut curpow10, i);
        assert_eq!(curpow10, *prevpow10.clone().mul_small(10));
        prevpow10 = curpow10;
    }
}

#[test]
fn shortest_sanity_test() {
    f64_shortest_sanity_test(format_shortest);
    f32_shortest_sanity_test(format_shortest);
    #[cfg(target_has_reliable_f16)]
    f16_shortest_sanity_test(format_shortest);
    more_shortest_sanity_test(format_shortest);
}

#[test]
#[cfg_attr(miri, ignore)] // Miri is too slow
#[cfg_attr(ferrocene_coverage, ignore = "test too slow with coverage enabled")]
fn exact_sanity_test() {
    // This test ends up running what I can only assume is some corner-ish case
    // of the `exp2` library function, defined in whatever C runtime we're
    // using. In VS 2013 this function apparently had a bug as this test fails
    // when linked, but with VS 2015 the bug appears fixed as the test runs just
    // fine.
    //
    // The bug seems to be a difference in return value of `exp2(-1057)`, where
    // in VS 2013 it returns a double with the bit pattern 0x2 and in VS 2015 it
    // returns 0x20000.
    //
    // For now just ignore this test entirely on MSVC as it's tested elsewhere
    // anyway and we're not super interested in testing each platform's exp2
    // implementation.
    if !cfg!(target_env = "msvc") {
        f64_exact_sanity_test(format_exact);
    }
    f32_exact_sanity_test(format_exact);

    #[cfg(target_has_reliable_f16)]
    f16_exact_sanity_test(format_exact);
}

#[test]
fn test_to_shortest_str() {
    to_shortest_str_test(format_shortest);
}

#[test]
fn test_to_shortest_exp_str() {
    to_shortest_exp_str_test(format_shortest);
}

#[test]
fn test_to_exact_exp_str() {
    to_exact_exp_str_test(format_exact);
}

#[test]
fn test_to_exact_fixed_str() {
    to_exact_fixed_str_test(format_exact);
}

#[test]
/// This test tries to detect incorrect rounding in `format_shortest`.
/// It tests:
/// - for each `0.1`, `0.01`, `0.001`, ..., for offset = 1..=20 digits:
/// - for each p = `10.pow(1)`, `10.pow(2)`, from k = -300..=300:
/// - Run `format_shortest` on `f = p - .0000....1`.
///
/// If the algorithm is incorrect, the annotated block will be hit, generating a coverage line, and
/// `blanket` will error that we have an unused annotation.
fn test_dragon_rounding_edge_cases() {
    let mut buf = Vec::with_capacity(MAX_SIG_DIGITS);
    let slice = buf.spare_capacity_mut();

    for k in -300..=300 {
        let p = 10f64.powi(k);
        if p.is_infinite() {
            continue;
        }
        let bits = p.to_bits();
        for offset in 1..=20u64 {
            let f = f64::from_bits(bits - offset);
            let (_negative, full_decoded) = decode(f);
            if let FullDecoded::Finite(decoded) = full_decoded {
                format_shortest(&decoded, slice);
            }
        }
    }
}
