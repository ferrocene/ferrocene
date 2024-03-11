macro_rules! x {
    ($($c:tt)*) => {
        $($c)ö*
    };
}

fn main() {
    x!(!); //~ ERROR macro expansion ends with an incomplete expression: expected expression
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
