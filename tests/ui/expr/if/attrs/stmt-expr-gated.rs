fn main() {
    let _ = #[deny(warnings)] if true { //~ ERROR attributes on expressions
    } else if false {
    } else {
    };
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_gvwd0kf72jt
// Attributes
