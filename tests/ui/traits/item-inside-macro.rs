//@ run-pass
// Issue #34183

macro_rules! foo {
    () => {
        fn foo() { }
    }
}

macro_rules! bar {
    () => {
        fn bar();
    }
}

trait Bleh {
    foo!();
    bar!();
}

struct Test;

impl Bleh for Test {
    fn bar() {}
}

fn main() {
    Test::bar();
    Test::foo();
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
