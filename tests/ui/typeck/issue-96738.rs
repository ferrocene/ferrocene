fn main() {
    Some.nonexistent_method(); //~ ERROR: no method named `nonexistent_method` found
    Some.nonexistent_field; //~ ERROR: no field `nonexistent_field`
}

// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
