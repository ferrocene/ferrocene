const C: i32 = 1i8; //~ ERROR mismatched types
const D: i8 = C; //~ ERROR mismatched types

const fn foo() {
    let c: i32 = 1i8; //~ ERROR mismatched types
    let d: i8 = c; //~ ERROR mismatched types
}

fn main() {
    let c: i32 = 1i8; //~ ERROR mismatched types
    let d: i8 = c; //~ ERROR mismatched types
}

// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
