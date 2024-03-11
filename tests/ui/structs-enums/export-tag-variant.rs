//@ run-pass
#![allow(non_camel_case_types)]
//@ pretty-expanded FIXME #23616

mod foo {
    pub enum t { t1, }
}

pub fn main() { let _v = foo::t::t1; }

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
