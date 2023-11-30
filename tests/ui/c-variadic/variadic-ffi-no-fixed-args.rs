extern "C" {
    fn foo(...);
//~^ ERROR C-variadic function must be declared with at least one named argument
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
