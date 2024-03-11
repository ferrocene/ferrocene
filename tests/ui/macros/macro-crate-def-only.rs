//@ run-pass
//@ aux-build:macro_crate_def_only.rs


#[macro_use] #[no_link]
extern crate macro_crate_def_only;

pub fn main() {
    assert_eq!(5, make_a_5!());
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
//
// ferrocene-annotations: fls_ch9nkxkloozv
// Attribute no_link
