macro_rules! foo {
    () => {
        #[cfg_attr(true, unknown)]
        //~^ ERROR cannot find attribute `unknown` in this scope
        fn foo() {}
    }
}

foo!();

fn main() {}

// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
