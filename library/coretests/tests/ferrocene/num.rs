macro_rules! nums_to_bytes_tests {
    ($($int:ty),*) => {
        $({
            let int = <$int>::MAX;
            let le_bytes = int.to_le_bytes();
            let be_bytes = int.to_be_bytes();

            assert_eq!(<$int>::from_le_bytes(le_bytes), int);
            assert_eq!(<$int>::from_be_bytes(be_bytes), int);
        })*
    }
}

#[test]
fn numbers_to_bytes() {
    nums_to_bytes_tests! {
        u8, u16, u32, u64, u128, usize,
        i8, i16, i32, i64, i128, isize,
        f32, f64
    }
}

#[test]
fn abs_diff_vectorization() {
    fn sad_iter(a: &[u8; 8], b: &[u8; 8]) -> u32 {
        a.iter().zip(b).map(|(&a, &b)| a.abs_diff(b) as u32).sum()
    }

    fn sad_loop(a: &[u8; 8], b: &[u8; 8]) -> u32 {
        let mut sum = 0;
        for i in 0..a.len() {
            sum += a[i].abs_diff(b[i]) as u32;
        }
        sum
    }

    let max_buf = [u8::MAX; 8];
    let min_buf = [u8::MIN; 8];

    assert_eq!(sad_iter(&max_buf, &min_buf), 8 * (u8::MAX as u32));
    assert_eq!(sad_loop(&max_buf, &min_buf), 8 * (u8::MAX as u32));
}

macro_rules! ilog2_loop {
    ($(($T:ty, $ilog2_max:expr) => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!(<$T>::MAX.ilog2(), $ilog2_max);
                for i in 0..=$ilog2_max {
                    let p = (2 as $T).pow(i as u32);
                    if p >= 2 {
                        assert_eq!((p - 1).ilog2(), i - 1);
                    }
                    assert_eq!(p.ilog2(), i);
                    if p >= 2 {
                        assert_eq!((p + 1).ilog2(), i);
                    }

                    // also check `x.ilog(2)`
                    if p >= 2 {
                        assert_eq!((p - 1).ilog(2), i - 1);
                    }
                    assert_eq!(p.ilog(2), i);
                    if p >= 2 {
                        assert_eq!((p + 1).ilog(2), i);
                    }
                }
            }
        )*
    };
}
ilog2_loop! {
    (u8, 7) => ilog2_u8,
    (u16, 15) => ilog2_u16,
    (u32, 31) => ilog2_u32,
    (u64, 63) => ilog2_u64,
    (u128, 127) => ilog2_u128,
    (i8, 6) => ilog2_i8,
    (i16, 14) => ilog2_i16,
    (i32, 30) => ilog2_i32,
    (i64, 62) => ilog2_i64,
    (i128, 126) => ilog2_i128,
}

macro_rules! nonpositive_ilog2 {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            #[should_panic = "argument of integer logarithm must be positive"]
            fn $fn() {
                let _ = (-1 as $T).ilog2();
            }
        )*
    };
}
nonpositive_ilog2! {
    i8 => nonpositive_ilog2_of_i8,
    i16 => nonpositive_ilog2_of_i16,
    i32 => nonpositive_ilog2_of_i32,
    i64 => nonpositive_ilog2_of_i64,
    i128 => nonpositive_ilog2_of_i128,
}

// covers:
// - `<core::num::niche_types::$T as core::hash::Hash>::hash`
// - `<core::num::niche_types::$T::new`
macro_rules! niche_types {
    ($($T:ident => $fn:ident $((invalid_value: $invalid_value:expr))?,)*) => {
        $(
            #[test]
            fn $fn() {
                use std::hash::{ Hasher, Hash, DefaultHasher };

                #[allow(unused_assignments, unused_mut)]
                let mut invalid_value = 0;
                $(invalid_value = $invalid_value;)*

                assert!(core::num::niche_types::$T::new(invalid_value).is_none());

                let int = core::num::niche_types::$T::new(1).unwrap();

                let mut hasher = DefaultHasher::new();
                int.hash(&mut hasher);
                let _ = hasher.finish();
            }
        )*
    }
}

niche_types! {
    NonZeroU8Inner => non_zero_u8_inner,
    NonZeroU16Inner => non_zero_u16_inner,
    NonZeroU32Inner => non_zero_u32_inner,
    NonZeroU64Inner => non_zero_u64_inner,
    NonZeroU128Inner => non_zero_u128_inner,
    NonZeroUsizeInner => non_zero_usize_inner,
    NonZeroI8Inner => non_zero_i8_inner,
    NonZeroI16Inner => non_zero_i16_inner,
    NonZeroI32Inner => non_zero_i32_inner,
    NonZeroI64Inner => non_zero_i64_inner,
    NonZeroI128Inner => non_zero_i128_inner,
    NonZeroIsizeInner => non_zero_isize_inner,
    Nanoseconds => nanoseconds(invalid_value: 1_000_000_000),
}

// covers `<core::num::nonzero::NonZero<T> as core::clone::Clone>::clone`.
#[test]
fn non_zero_clone() {
    let val = core::num::NonZero::<u8>::new(1).unwrap();
    assert_eq!(Clone::clone(&val), val);
}

// covers `<core::num::niche_types::Nanoseconds as core::default::Default>::default`.
#[test]
fn default_nanoseconds() {
    assert_eq!(core::num::niche_types::Nanoseconds::new(0).unwrap(), Default::default());
}

// covers `core::num::<T>::overflowing_neg`.
macro_rules! int_overflowing_neg {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                let (out, did_overflow) = <$T>::MIN.overflowing_neg();
                assert!(did_overflow);
                assert_eq!(out, <$T>::MIN)
            }
        )*
    };
}

int_overflowing_neg! {
    i8 => i8_overflowing_neg,
    i16 => i16_overflowing_neg,
    i32 => i32_overflowing_neg,
    i64 => i64_overflowing_neg,
    i128 => i128_overflowing_neg,
    isize => isize_overflowing_neg,
}

// Covers core::num::int_log10::usize
#[test]
fn int_log10_usize() {
    let a = core::num::NonZeroUsize::new(100).unwrap();
    assert_eq!(a.ilog10(), 2);
}

// Covers
// - core::num::<impl u8>::checked_pow
// - core::num::<impl u16>::checked_pow
// - core::num::<impl u32>::checked_pow
// - core::num::<impl u64>::checked_pow
// - core::num::<impl u128>::checked_pow
// - core::num::<impl usize>::checked_pow
macro_rules! test_uint_checked_pow {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!(<$T>::checked_pow(2, 5), Some(32));
                assert_eq!(<$T>::checked_pow(0, 0), Some(1));
                assert_eq!(<$T>::checked_pow(<$T>::MAX, 2), None);
            }
        )*
    };
}
test_uint_checked_pow! {
    u8 => u8_checked_pow,
    u16 => u16_checked_pow,
    u32 => u32_checked_pow,
    u64 => u64_checked_pow,
    u128 => u128_checked_pow,
    usize => usize_checked_pow,
}

// Covers
// - core::num::<impl i8>::ilog
// - core::num::<impl i16>::ilog
// - core::num::<impl i32>::ilog
// - core::num::<impl i64>::ilog
// - core::num::<impl i128>::ilog
// - core::num::<impl isize>::ilog
macro_rules! test_int_ilog_panic_nonpositive_argument {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            #[should_panic = "argument of integer logarithm must be positive"]
            fn $fn() {
                assert_eq!(<$T>::ilog(-1, 2), 0);
            }
        )*
    };
}
test_int_ilog_panic_nonpositive_argument! {
    i8 => i8_ilog_panic_nonpositive_argument,
    i16 => i16_ilog_panic_nonpositive_argument,
    i32 => i32_ilog_panic_nonpositive_argument,
    i64 => i64_ilog_panic_nonpositive_argument,
    i128 => i128_ilog_panic_nonpositive_argument,
    isize => isize_ilog_panic_nonpositive_argument,
}

// Cover `core::fmt::num::imp::exp_u64`
#[test]
fn fmt_num_exp_u64() {
    assert_eq!(format!("{:e}", 100_u64), "1e2");
    assert_eq!(format!("{:.1e}", 100_u64), "1.0e2");
    assert_eq!(format!("{:.1e}", 155_u64), "1.6e2");
    assert_eq!(format!("{:.1e}", 199_u64), "2.0e2");
    assert_eq!(format!("{:.1e}", 999_u64), "1.0e3");
    assert_eq!(format!("{:.5e}", 100_u64), "1.00000e2");
    assert_eq!(format!("{:.5e}", -100_i64), "-1.00000e2");
    assert_eq!(format!("{:+.5e}", 100_u64), "+1.00000e2");
    assert_eq!(format!("{:e}", 1_000_000_000_000_u64), "1e12");
}

// Cover `core::fmt::num::exp_u128`
#[test]
fn fmt_num_exp_u128() {
    assert_eq!(format!("{:e}", 100_u128), "1e2");
    assert_eq!(format!("{:.1e}", 100_u128), "1.0e2");
    assert_eq!(format!("{:.1e}", 155_u128), "1.6e2");
    assert_eq!(format!("{:.1e}", 199_u128), "2.0e2");
    assert_eq!(format!("{:.1e}", 999_u128), "1.0e3");
    assert_eq!(format!("{:.5e}", 100_u128), "1.00000e2");
    assert_eq!(format!("{:.5e}", -100_i128), "-1.00000e2");
    assert_eq!(format!("{:+.5e}", 100_u128), "+1.00000e2");
    assert_eq!(format!("{:e}", 1_000_000_000_000_u128), "1e12");
}

// Covers
// - `core::num::<impl i128>::abs_diff`
// - `core::num::<impl i16>::abs_diff`
// - `core::num::<impl i32>::abs_diff`
// - `core::num::<impl i64>::abs_diff`
// - `core::num::<impl i8>::abs_diff`
macro_rules! test_int_abs_diff {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!((10 as $T).abs_diff(-10), 20);
                assert_eq!((-10 as $T).abs_diff(10), 20);
            }
        )*
    };
}
test_int_abs_diff! {
    i8 => i8_abs_diff,
    i16 => i16_abs_diff,
    i32 => i32_abs_diff,
    i64 => i64_abs_diff,
    i128 => i128_abs_diff,
    isize => isize_abs_diff,
}

// Covers
// - `core::convert::num::<impl core::convert::TryFrom<i128> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<i16> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<i32> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<i64> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<i8> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<u128> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<u16> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<u32> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<u64> for bool>::try_from`
// - `core::convert::num::<impl core::convert::TryFrom<u8> for bool>::try_from`
macro_rules! test_bool_try_from_int {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!(bool::try_from(0 as $T), Ok(false));
                assert_eq!(bool::try_from(1 as $T), Ok(true));
                assert!(bool::try_from(2 as $T).is_err());
            }
        )*
    };
}
test_bool_try_from_int! {
    u8 => bool_try_from_u8,
    u16 => bool_try_from_u16,
    u32 => bool_try_from_u32,
    u64 => bool_try_from_u64,
    u128 => bool_try_from_u128,
    i8 => bool_try_from_i8,
    i16 => bool_try_from_i16,
    i32 => bool_try_from_i32,
    i64 => bool_try_from_i64,
    i128 => bool_try_from_i128,
}

// covers: `<core::num::niche_types::$T as core::fmt::Debug>::fmt`
macro_rules! test_niche_types_debug_fmt {
    ($($fn:ident => $T:ident : $val:literal == $str:literal,)*) => { $(
        #[test]
        fn $fn() {
            let val = <core::num::niche_types::$T>::new($val).unwrap();

            assert_eq!(format!("{val:?}"), $str);
        }
    )*};
}
test_niche_types_debug_fmt!(
    non_zero_u8_inner_debug_fmt => NonZeroU8Inner: 5 == "5",
    non_zero_u16_inner_debug_fmt => NonZeroU16Inner: 5 == "5",
    non_zero_u32_inner_debug_fmt => NonZeroU32Inner: 5 == "5",
    non_zero_u64_inner_debug_fmt => NonZeroU64Inner: 5 == "5",
    non_zero_usize_inner_debug_fmt => NonZeroUsizeInner: 5 == "5",
    non_zero_i8_inner_debug_fmt => NonZeroI8Inner: 5 == "5",
    non_zero_i16_inner_debug_fmt => NonZeroI16Inner: 5 == "5",
    non_zero_i32_inner_debug_fmt => NonZeroI32Inner: 5 == "5",
    non_zero_i64_inner_debug_fmt => NonZeroI64Inner: 5 == "5",
    non_zero_isize_inner_debug_fmt => NonZeroIsizeInner: 5 == "5",
);

// covers: `<$T as core::pat::RangePattern>::sub_one`
macro_rules! test_range_pattern_sub_one {
    ($(( $fn:ident, $fn_panic:ident ) => $T:ty,)*) => {
        $(
            #[test]
            fn $fn() {
                assert_eq!(0 as $T, core::pat::RangePattern::sub_one(1 as $T));
            }

            #[test]
            #[should_panic = "exclusive range end at minimum value of type"]
            fn $fn_panic() {
                core::pat::RangePattern::sub_one(<$T>::MIN);
            }
        )*
    };
}
test_range_pattern_sub_one! {
    (range_pattern_sub_one_u8, range_pattern_sub_one_panic_u8) => u8,
    (range_pattern_sub_one_u16, range_pattern_sub_one_panic_u16) => u16,
    (range_pattern_sub_one_u32, range_pattern_sub_one_panic_u32) => u32,
    (range_pattern_sub_one_u64, range_pattern_sub_one_panic_u64) => u64,
    (range_pattern_sub_one_u128, range_pattern_sub_one_panic_u128) => u128,
    (range_pattern_sub_one_usize, range_pattern_sub_one_panic_usize) => usize,
    (range_pattern_sub_one_i8, range_pattern_sub_one_panic_i8) => i8,
    (range_pattern_sub_one_i16, range_pattern_sub_one_panic_i16) => i16,
    (range_pattern_sub_one_i32, range_pattern_sub_one_panic_i32) => i32,
    (range_pattern_sub_one_i64, range_pattern_sub_one_panic_i64) => i64,
    (range_pattern_sub_one_i128, range_pattern_sub_one_panic_i128) => i128,
    (range_pattern_sub_one_isize, range_pattern_sub_one_panic_isize) => isize,
}

// covers `<core::num::error::ParseIntError as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "cannot parse integer from empty string"]
fn test_parse_int_error_empty_display_fmt() {
    if let Err(e) = "".parse::<u8>() {
        panic!("{e}")
    }
}

// covers `<core::num::error::ParseIntError as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "invalid digit found in string"]
fn test_parse_int_error_invalid_digit_display_fmt() {
    if let Err(e) = "hello".parse::<u8>() {
        panic!("{e}")
    }
}

// covers `<core::num::error::ParseIntError as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "number too large to fit in target type"]
fn test_parse_int_error_pos_overflow_display_fmt() {
    if let Err(e) = "256".parse::<u8>() {
        panic!("{e}")
    }
}

// covers `<core::num::error::ParseIntError as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "number too small to fit in target type"]
fn test_parse_int_error_neg_overflow_display_fmt() {
    if let Err(e) = "-129".parse::<i8>() {
        panic!("{e}")
    }
}

// covers `<core::num::error::ParseIntError as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "number would be zero for non-zero type"]
fn test_parse_int_error_zero_display_fmt() {
    if let Err(e) = "0".parse::<core::num::NonZeroU8>() {
        panic!("{e}")
    }
}

// covers `<core::num::error::TryFromIntError as core::fmt::Display>::fmt`
#[test]
#[should_panic = "out of range integral type conversion attempted"]
fn test_try_from_int_error_display_fmt() {
    if let Err(e) = u8::try_from(256u16) {
        panic!("{e}")
    }
}

// covers `<core::num::nonzero::NonZero<T> as core::cmp::PartialEq>::ne`.
#[test]
fn non_zero_ne() {
    let lhs = core::num::NonZero::<u8>::new(1).unwrap();
    let rhs = core::num::NonZero::<u8>::new(2).unwrap();
    assert!(lhs != rhs);
    assert!(!(lhs != lhs));
}

// covers
// - <core::num::saturating::Saturating<T> as core::ops::arith::AddAssign>::add_assign
// - <core::num::saturating::Saturating<T> as core::ops::arith::DivAssign>::div_assign
// - <core::num::saturating::Saturating<T> as core::ops::arith::MulAssign>::mul_assign
// - <core::num::saturating::Saturating<T> as core::ops::arith::Neg>::neg
// - <core::num::saturating::Saturating<T> as core::ops::arith::Rem>::rem
// - <core::num::saturating::Saturating<T> as core::ops::arith::RemAssign<T>>::rem_assign
// - <core::num::saturating::Saturating<T> as core::ops::arith::RemAssign>::rem_assign
// - <core::num::saturating::Saturating<T> as core::ops::arith::SubAssign>::sub_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitAnd>::bitand
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitAndAssign<T>>::bitand_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitAndAssign>::bitand_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitOr>::bitor
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitOrAssign<T>>::bitor_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitOrAssign>::bitor_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitXor>::bitxor
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitXorAssign<T>>::bitxor_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::BitXorAssign>::bitxor_assign
// - <core::num::saturating::Saturating<T> as core::ops::bit::Not>::not
macro_rules! test_saturating_ops {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                use core::num::Saturating;


                // add_assign (Saturating)
                {
                    let mut max = Saturating(<$T>::MAX);
                    max += Saturating(1 as $T);
                    assert_eq!(max, Saturating(<$T>::MAX), "add_assign (Saturating)");
                }

                // add_assign
                {
                    let mut max = Saturating(<$T>::MAX);
                    max += 1 as $T;
                    assert_eq!(max, Saturating(<$T>::MAX), "add_assign");
                }

                // add (Saturating)
                {
                    let added_max = Saturating(<$T>::MAX) + Saturating(1 as $T);
                    assert_eq!(added_max, Saturating(<$T>::MAX), "add (Saturating)");
                }

                // sub_assign (Saturating)
                {
                    let mut min = Saturating(<$T>::MIN);
                    min -= Saturating(1 as $T);
                    assert_eq!(min, Saturating(<$T>::MIN), "sub_assign (Saturating)");
                }

                // sub_assign
                {
                    let mut min = Saturating(<$T>::MIN);
                    min -= 1 as $T;
                    assert_eq!(min, Saturating(<$T>::MIN), "sub_assign");
                }

                // sub (Saturating)
                {
                    let subbed_min = Saturating(<$T>::MIN) - Saturating(1 as $T);
                    assert_eq!(subbed_min, Saturating(<$T>::MIN), "sub (Saturating)");
                }

                // mul_assign (Saturating)
                {
                    let mut max = Saturating(<$T>::MAX);
                    max *= Saturating(2);
                    assert_eq!(max, Saturating(<$T>::MAX), "mul_assign (Saturating)");
                }

                // mul_assign
                {
                    let mut max = Saturating(<$T>::MAX);
                    max *= 2;
                    assert_eq!(max, Saturating(<$T>::MAX), "mul_assign");
                }

                // div_assign (Saturating)
                {
                    let mut max = Saturating(<$T>::MAX);
                    max /= Saturating(2);
                    assert_eq!(max, Saturating(<$T>::MAX / 2), "div_assign (Saturating)");
                }

                // div_assign
                {
                    let mut max = Saturating(<$T>::MAX);
                    max /= 2;
                    assert_eq!(max, Saturating(<$T>::MAX / 2), "div_assign");
                }

                // rem_assign (Saturating)
                {
                    let mut suspect = Saturating(15 as $T);
                    suspect %= Saturating(2);
                    assert_eq!(suspect, Saturating(1), "rem_assign (Saturating)");
                }

                // rem_assign
                {
                    let mut suspect = Saturating(15 as $T);
                    suspect %= 2;
                    assert_eq!(suspect, Saturating(1), "rem_assign");
                }

                // rem (Saturating)
                {
                    let suspect = Saturating(15 as $T) % Saturating(2 as $T);
                    assert_eq!(suspect, Saturating(1), "rem (Saturating)");
                }

                // bit_or (Saturating)
                {
                    let suspect = Saturating(5 as $T) | Saturating(2 as $T);
                    assert_eq!(suspect, Saturating(7), "bit_or (Saturating)");
                }

                // bit_or_assign (Saturating)
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect |= Saturating(2 as $T);
                    assert_eq!(suspect, Saturating(7), "bit_or_assign (Saturating)");
                }

                // bit_or_assign
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect |= 2 as $T;
                    assert_eq!(suspect, Saturating(7), "bit_or_assign");
                }

                // bit_and (Saturating)
                {
                    let suspect = Saturating(5 as $T) & Saturating(2 as $T);
                    assert_eq!(suspect, Saturating(0), "bit_and (Saturating)");
                }

                // bit_and_assign (Saturating)
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect &= Saturating(2 as $T);
                    assert_eq!(suspect, Saturating(0), "bit_and_assign (Saturating)");
                }

                // bit_and_assign
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect &= 2 as $T;
                    assert_eq!(suspect, Saturating(0), "bit_and_assign");
                }

                // bit_xor (Saturating)
                {
                    let suspect = Saturating(5 as $T) ^ Saturating(1 as $T);
                    assert_eq!(suspect, Saturating(4), "bit_xor");
                }

                // bit_xor_assign (Saturating)
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect ^= Saturating(1 as $T);
                    assert_eq!(suspect, Saturating(4), "bit_xor_assign (Saturating)");
                }

                // bit_xor_assign
                {
                    let mut suspect = Saturating(5 as $T);
                    suspect ^= 1 as $T;
                    assert_eq!(suspect, Saturating(4), "bit_xor_assign");
                }

                // not
                {
                    let suspect = Saturating(<$T>::MAX);
                    assert_eq!(!suspect, Saturating(<$T>::MIN), "not");
                }
            }

        )*
    };
}
test_saturating_ops! {
    i128 => saturating_i128_ops,
    i64 => saturating_i64_ops,
    i32 => saturating_i32_ops,
    i8 => saturating_i8_ops,
    isize => saturating_isize_ops,
    u128 => saturating_u128_ops,
    u64 => saturating_u64_ops,
    u32 => saturating_u32_ops,
    u8 => saturating_u8_ops,
    usize => saturating_usize_ops,
}

// covers
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::AddAssign>::add_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::DivAssign>::div_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::MulAssign>::mul_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::Neg>::neg
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::Rem>::rem
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::RemAssign<T>>::rem_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::RemAssign>::rem_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::arith::SubAssign>::sub_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitAnd>::bitand
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitAndAssign<T>>::bitand_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitAndAssign>::bitand_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitOr>::bitor
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitOrAssign<T>>::bitor_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitOrAssign>::bitor_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitXor>::bitxor
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitXorAssign<T>>::bitxor_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::BitXorAssign>::bitxor_assign
// - <core::num::wrapping::Wrapping<T> as core::ops::bit::Not>::not
// - <core::num::saturating::Saturating<T> as core::ops::arith::Neg>::neg
macro_rules! test_wrapping_ops {
    ($($T:ty => $fn:ident,)*) => {
        $(
            #[test]
            fn $fn() {
                use core::num::Wrapping;


                // add_assign (Wrapping)
                {
                    let mut max = Wrapping(<$T>::MAX);
                    max += Wrapping(1 as $T);
                    assert_eq!(max, Wrapping(<$T>::MIN), "add_assign (Wrapping)");
                }

                // add_assign
                {
                    let mut max = Wrapping(<$T>::MAX);
                    max += 1 as $T;
                    assert_eq!(max, Wrapping(<$T>::MIN), "add_assign");
                }

                // add (Wrapping)
                {
                    let added_max = Wrapping(<$T>::MAX) + Wrapping(1 as $T);
                    assert_eq!(added_max, Wrapping(<$T>::MIN), "add (Wrapping)");
                }

                // sub_assign (Wrapping)
                {
                    let mut min = Wrapping(<$T>::MIN);
                    min -= Wrapping(1 as $T);
                    assert_eq!(min, Wrapping(<$T>::MAX), "sub_assign (Wrapping)");
                }

                // sub_assign (Wrapping)
                {
                    let mut min = Wrapping(<$T>::MIN);
                    min -= 1 as $T;
                    assert_eq!(min, Wrapping(<$T>::MAX), "sub_assign (Wrapping)");
                }

                // sub (Wrapping)
                {
                    let subbed_min = Wrapping(<$T>::MIN) - Wrapping(1 as $T);
                    assert_eq!(subbed_min, Wrapping(<$T>::MAX), "sub (Wrapping)");
                }

                // mul_assign (Wrapping)
                {
                    let mut max = Wrapping(24 as $T);
                    max *= Wrapping(2); // Other functions test the correctness of mul in general, just worry about covering this convienence wrapper
                    assert_eq!(max, Wrapping(48), "mul_assign (Wrapping)");
                }

                // mul_assign
                {
                    let mut max = Wrapping(24 as $T);
                    max *= 2; // Other functions test the correctness of mul in general, just worry about covering this convienence wrapper
                    assert_eq!(max, Wrapping(48), "mul_assign");
                }

                // div_assign (Wrapping)
                {
                    let mut max = Wrapping(<$T>::MAX);
                    max /= Wrapping(2);
                    assert_eq!(max, Wrapping(<$T>::MAX / 2), "div_assign (Wrapping)");
                }

                // div_assign
                {
                    let mut max = Wrapping(<$T>::MAX);
                    max /= 2;
                    assert_eq!(max, Wrapping(<$T>::MAX / 2), "div_assign");
                }

                // rem_assign (Wrapping)
                {
                    let mut suspect = Wrapping(15 as $T);
                    suspect %= Wrapping(2);
                    assert_eq!(suspect, Wrapping(1), "rem_assign (Wrapping)");
                }

                // rem_assign
                {
                    let mut suspect = Wrapping(15 as $T);
                    suspect %= 2;
                    assert_eq!(suspect, Wrapping(1), "rem_assign");
                }

                // rem (Wrapping)
                {
                    let suspect = Wrapping(15 as $T) % Wrapping(2 as $T);
                    assert_eq!(suspect, Wrapping(1), "rem (Wrapping)");
                }

                // bit_or (Wrapping)
                {
                    let suspect = Wrapping(5 as $T) | Wrapping(2 as $T);
                    assert_eq!(suspect, Wrapping(7), "bit_or (Wrapping)");
                }

                // bit_or_assign (Wrapping)
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect |= Wrapping(2 as $T);
                    assert_eq!(suspect, Wrapping(7), "bit_or_assign (Wrapping)");
                }


                // bit_or_assign
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect |= 2 as $T;
                    assert_eq!(suspect, Wrapping(7), "bit_or_assign");
                }

                // bit_and (Wrapping)
                {
                    let suspect = Wrapping(5 as $T) & Wrapping(2 as $T);
                    assert_eq!(suspect, Wrapping(0), "bit_and (Wrapping)");
                }

                // bit_and_assign (Wrapping)
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect &= Wrapping(2 as $T);
                    assert_eq!(suspect, Wrapping(0), "bit_and_assign (Wrapping)");
                }

                // bit_and_assign (Wrapping)
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect &= 2 as $T;
                    assert_eq!(suspect, Wrapping(0), "bit_and_assign");
                }

                // bit_xor (Wrapping)
                {
                    let suspect = Wrapping(5 as $T) ^ Wrapping(1 as $T);
                    assert_eq!(suspect, Wrapping(4), "bit_xor (Wrapping)");
                }

                // bit_xor_assign (Wrapping)
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect ^= Wrapping(1 as $T);
                    assert_eq!(suspect, Wrapping(4), "bit_xor_assign (Wrapping)");
                }

                // bit_xor_assign
                {
                    let mut suspect = Wrapping(5 as $T);
                    suspect ^= 1 as $T;
                    assert_eq!(suspect, Wrapping(4), "bit_xor_assign");
                }

                // not
                {
                    let suspect = Wrapping(<$T>::MAX);
                    assert_eq!(!suspect, Wrapping(<$T>::MIN), "not");
                }
            }

        )*
    };
}
test_wrapping_ops! {
    i128 => wrapping_i128_ops,
    i64 => wrapping_i64_ops,
    i32 => wrapping_i32_ops,
    i8 => wrapping_i8_ops,
    isize => wrapping_isize_ops,
    u128 => wrapping_u128_ops,
    u64 => wrapping_u64_ops,
    u32 => wrapping_u32_ops,
    u8 => wrapping_u8_ops,
    usize => wrapping_usize_ops,
}

// Covers `<core::num::saturating::Saturating<T> as core::ops::arith::Neg>::neg`
#[test]
fn test_saturating_neg() {
    use core::num::Saturating;
    assert_eq!(-Saturating(42 as i128), Saturating(-42 as i128));
    assert_eq!(-Saturating(42 as i64), Saturating(-42 as i64));
    assert_eq!(-Saturating(42 as i32), Saturating(-42 as i32));
    assert_eq!(-Saturating(42 as i8), Saturating(-42 as i8));
}
