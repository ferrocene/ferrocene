// run-rustfix
fn main() {
    let mut _foo: i32 = 1;
    _foo: i32 = 4; //~ ERROR type ascription is experimental
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
