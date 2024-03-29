//@ run-pass
//@ compile-flags: -C debug_assertions=no
//@ ignore-emscripten dies with an LLVM error

fn main() {
    for i in 129..256 {
        assert_eq!((i as u8).next_power_of_two(), 0);
    }

    assert_eq!(((1u16 << 15) + 1).next_power_of_two(), 0);
    assert_eq!(((1u32 << 31) + 1).next_power_of_two(), 0);
    assert_eq!(((1u64 << 63) + 1).next_power_of_two(), 0);
    assert_eq!(((1u128 << 127) + 1).next_power_of_two(), 0);
}

// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
