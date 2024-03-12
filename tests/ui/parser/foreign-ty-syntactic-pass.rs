//@ check-pass

fn main() {}

#[cfg(FALSE)]
extern "C" {
    type A: Ord;
    type A<'a> where 'a: 'static;
    type A<T: Ord> where T: 'static;
    type A = u8;
    type A<'a: 'static, T: Ord + 'static>: Eq + PartialEq where T: 'static + Copy = Vec<u8>;
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
