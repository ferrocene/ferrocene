// Test macro_undefined issue

mod m {
    #[macro_export]
    macro_rules! kl {
        () => ()
    }
}

fn main() {
    k!(); //~ ERROR cannot find
    kl!();
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
