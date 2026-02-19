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
