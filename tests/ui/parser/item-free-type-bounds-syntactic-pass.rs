//@ check-pass

fn main() {}

#[cfg(false)]
fn syntax() {
    type A: Ord;
    type B: Ord = u8;
    type C: Ord where 'static: 'static = u8;
    type D<_T>: Ord;
    type E<_T>: Ord = u8;
    type F<_T>: Ord where 'static: 'static = u8;
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
