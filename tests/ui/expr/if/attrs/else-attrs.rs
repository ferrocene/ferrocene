#[cfg(FALSE)]
fn if_else_parse_error() {
    if true {
    } #[attr] else if false { //~ ERROR expected
    }
}

#[cfg(FALSE)]
fn else_attr_ifparse_error() {
    if true {
    } else #[attr] if false { //~ ERROR outer attributes are not allowed
    } else {
    }
}

#[cfg(FALSE)]
fn else_parse_error() {
    if true {
    } else if false {
    } #[attr] else { //~ ERROR expected
    }
}

fn main() {
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_u1afezy1ye99
// Conditional Compilation
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
