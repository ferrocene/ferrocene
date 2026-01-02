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

// covers `<core::num::niche_types::Nanoseconds as core::default::Default>::default`.
#[test]
fn default_nanoseconds() {
    assert_eq!(core::num::niche_types::Nanoseconds::new(0).unwrap(), Default::default());
}
