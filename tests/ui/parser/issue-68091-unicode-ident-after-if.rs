macro_rules! x {
    ($($c:tt)*) => {
        $($c)ö* {}
        //~^ ERROR missing condition for `if` expression
    };
}

fn main() {
    x!(if);
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
