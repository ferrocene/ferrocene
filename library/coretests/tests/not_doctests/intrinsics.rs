use core::intrinsics::fallback::FunnelShift;

macro_rules! test_unchecked_funnel_shift {
    ($lhs:literal, $rhs:literal, $shift:literal, $res_shl:literal, $res_shr:literal) => {
        unsafe {
            // shift left
            assert_eq!($lhs.unchecked_funnel_shl($rhs, $shift), $res_shl);
            assert_eq!($lhs.unchecked_funnel_shl($rhs, 0), $lhs);

            // shift right
            assert_eq!($lhs.unchecked_funnel_shr($rhs, $shift), $res_shr);
            assert_eq!($lhs.unchecked_funnel_shr($rhs, 0), $rhs);
        }
    };
}

#[test]
fn unchecked_funnel_shift_u8() {
    test_unchecked_funnel_shift!(0x82u8, 0x36u8, 2, 0x8, 0x8d);
}

#[test]
fn unchecked_funnel_shift_u16() {
    test_unchecked_funnel_shift!(0xa003u16, 0x2deu16, 4, 0x30, 0x302d);
}

#[test]
fn unchecked_funnel_shift_u32() {
    test_unchecked_funnel_shift!(0x10000b3u32, 0x2fe78e45u32, 8, 0xb32f, 0xb32fe78e);
}

#[test]
fn unchecked_funnel_shift_u64() {
    test_unchecked_funnel_shift!(
        0xaa00000000006e1u64,
        0x2fe78e45983acd98u64,
        12,
        0x6e12fe,
        0x6e12fe78e45983ac
    );
}

#[test]
fn unchecked_funnel_shift_u128() {
    test_unchecked_funnel_shift!(
        0x13f40000000000000000000000004f76u128,
        0x2fe78e45983acd98039000008736273u128,
        16,
        0x4f7602fe,
        0x4f7602fe78e45983acd9803900000873
    );
}

#[test]
fn unchecked_funnel_shift_usize() {
    test_unchecked_funnel_shift!(
        0xaa00000000006e1usize,
        0x2fe78e45983acd98usize,
        12,
        0x6e12fe,
        0x6e12fe78e45983ac
    );
}
