pub mod m {
    pub struct S {
        pub visible: bool,
        a: (),
        b: (),
        c: (),
        d: (),
        e: (),
    }
}

fn main() {
    let _ = m::S { //~ ERROR cannot construct `S` with struct literal syntax due to private fields
        visible: true,
        a: (),
        b: (),
    };
}

// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_8tsynkj2cufj
// Struct Expressions
