//@ check-fail
//@ compile-flags: -C link-dead-code=invalid
//~? incorrect value

fn main() {}

// ferrocene-annotations: um_rustc_C_link_dead_code
