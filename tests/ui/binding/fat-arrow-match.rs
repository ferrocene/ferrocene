//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]

enum color {
    red,
    green,
    blue
}

pub fn main() {
    println!("{}", match color::red {
        color::red => { 1 }
        color::green => { 2 }
        color::blue => { 3 }
    });
}

// ferrocene-annotations: fls_d44aflefat88
// Path pattern matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path patterns
