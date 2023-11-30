// #[test] attribute is not allowed on associated functions or methods
// reworded error message
// compile-flags:--test

struct A {}

impl A {
    #[test]
    fn new() -> A {
        //~^ ERROR `#[test]` attribute is only allowed on non associated functions
        A {}
    }
    #[test]
    fn recovery_witness() -> A {
        //~^ ERROR `#[test]` attribute is only allowed on non associated functions
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
