//@ check-pass
//@ edition:2018

mod my {
    pub mod sub {
        pub fn bar() {}
    }
}

mod sub {
    pub fn bar() {}
}

fn foo() {
    use my::sub;
    {
        use sub::bar; // OK
    }
}

fn main() {}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
