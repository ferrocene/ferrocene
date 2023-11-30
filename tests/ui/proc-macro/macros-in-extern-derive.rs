extern "C" {
    #[derive(Copy)] //~ ERROR `derive` may only be applied to `struct`s, `enum`s and `union`s
    fn f();
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
