//@ compile-flags:--test

struct A {}

impl A {
    #[test]
    //~^ ERROR the `#[test]` attribute may only be used on a non-associated function
    fn new() -> A {
        A {}
    }
    #[test]
    //~^ ERROR the `#[test]` attribute may only be used on a non-associated function
    fn recovery_witness() -> A {
        A {}
    }
}

#[test]
fn test() {
    let _ = A::new();
}

fn main() {}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: um_rustc_test
