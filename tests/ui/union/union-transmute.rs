//@ run-pass

union U {
    a: (u8, u8),
    b: u16,
}

union W {
    a: u32,
    b: f32,
}

fn main() {
    unsafe {
        let mut u = U { a: (1, 1) };
        assert_eq!(u.b, (1 << 8) + 1);
        u.b = (2 << 8) + 2;
        assert_eq!(u.a, (2, 2));

        let mut w = W { a: 0b0_11111111_00000000000000000000000 };
        assert_eq!(w.b, f32::INFINITY);
        w.b = f32::NEG_INFINITY;
        assert_eq!(w.a, 0b1_11111111_00000000000000000000000);
    }
}

// ferrocene-annotations: fls_8wnyln2nmg4y
// Unsafe Blocks
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_k64tfywtn0g8
// Tuple Expressions
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
