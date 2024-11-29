//@ run-pass
#![allow(dead_code)]
#![allow(non_camel_case_types)]



mod m1 {
    pub enum foo { foo1, foo2, }
}

fn bar(x: m1::foo) { match x { m1::foo::foo1 => { } m1::foo::foo2 => { } } }

pub fn main() { }

// ferrocene-annotations: fls_d44aflefat88
// Path pattern matching
//
// ferrocene-annotations: fls_uloyjbaso8pz
// Path patterns
