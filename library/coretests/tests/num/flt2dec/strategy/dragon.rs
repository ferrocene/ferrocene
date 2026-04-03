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
fn exhaustive_test_dragon_f32() {
    // We only need to test finite non-zero numbers.
    let mut buf = Vec::with_capacity(MAX_SIG_DIGITS);
    let slice = buf.spare_capacity_mut();
    let mut f = f32::MIN;
    let mut i = 0;
    eprintln!("starting");
    while f != f32::INFINITY {
        if i % 1_000_000 == 0 {
            eprintln!("i={i}");
        }
        let (_negative, full_decoded) = decode(f);
        if let FullDecoded::Finite(decoded) = full_decoded {
            format_shortest(&decoded, slice);
        }
        f = f.next_up();
        i += 1;
    }
}

fn exhaustive_test_dragon_f64_range(start: f64, end: f64, thread_name: &str) {
    // See `flt2dec::to_shortest_exp_str`.
    // We only need to test finite non-zero numbers.
    let mut buf = Vec::with_capacity(MAX_SIG_DIGITS);
    let slice = buf.spare_capacity_mut();
    let mut f = start;
    let mut i = 0;
    while f != end {
        if i % 100_000_000 == 0 {
            eprintln!("{thread_name}: i={i}");
        }
        let (_negative, full_decoded) = decode(f);
        if let FullDecoded::Finite(decoded) = full_decoded {
            format_shortest(&decoded, slice);
        }
        f = f.next_up();
        i += 1;
    }
}

#[test]
fn exhaustive_test_dragon_f64() {
    let cores = 31; // change this if you're running on a weaker machine
    let bucket_size = u64::MAX / cores;
    let ranges: Vec<_> = (0..cores).map(|bucket| (bucket*bucket_size, (bucket+1)*bucket_size)).collect();
    eprintln!("testing ranges: {ranges:?}");
    // Start all the threads.
    let handles: Vec<_> = ranges.into_iter().enumerate().map(|(i, (range_start, range_end))| {
        std::thread::spawn(move || {
            exhaustive_test_dragon_f64_range(f64::from_bits(range_start), f64::from_bits(range_end), &i.to_string());
        })
    }).collect();
    for handle in handles {
        if let Err(e) = handle.join() {
            eprintln!("error joining handle! {e:?}");
        } else {
            eprintln!("joined handle");
        }
    }
}
