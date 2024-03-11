//@ run-pass

mod a {
    pub trait Foo {
        fn foo() -> Self;
    }

    impl Foo for isize {
        fn foo() -> isize {
            3
        }
    }

    impl Foo for usize {
        fn foo() -> usize {
            5
        }
    }
}

pub fn main() {
    let x: isize = a::Foo::foo();
    let y: usize = a::Foo::foo();
    assert_eq!(x, 3);
    assert_eq!(y, 5);
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
