//@ check-pass

fn main() {
    #[allow(unused_variables)]
    if true {
        let a = 1;
    } else if false {
        let b = 1;
    } else {
        let c = 1;
    }
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_1pg5ig740tg1
// Expression Statements
