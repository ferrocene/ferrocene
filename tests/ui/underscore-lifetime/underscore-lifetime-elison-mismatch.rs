fn foo(x: &mut Vec<&'_ u8>, y: &'_ u8) { x.push(y); }
//~^ ERROR lifetime may not live long enough

fn main() {}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_l9ebxrlxyawd
// Lifetime Elision
//
// ferrocene-annotations: fls_hethxxbcg7ja
// Function Lifetime Elision
