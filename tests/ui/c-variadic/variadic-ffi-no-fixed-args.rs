//@ build-pass

// Supported since C23
// https://www.open-std.org/jtc1/sc22/wg14/www/docs/n2975.pdf
extern "C" {
    fn foo(...);
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
