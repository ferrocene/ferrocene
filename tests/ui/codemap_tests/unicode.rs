//@ revisions: normal expanded
//@[expanded] check-pass
//@[expanded]compile-flags: -Zunpretty=expanded

extern "路濫狼á́́" fn foo() {} //[normal]~ ERROR invalid ABI

fn main() { }

// ferrocene-annotations: fls_2i089jvv8j5g
// Character Set
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_usgd0xlijoxv
// ABI
