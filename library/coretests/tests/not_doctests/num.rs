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
