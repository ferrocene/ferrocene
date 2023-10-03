extern "C" {
    fn foo<const X: usize>(); //~ ERROR foreign items may not have const parameters

    fn bar<T, const X: usize>(_: T); //~ ERROR foreign items may not have type or const parameters
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
