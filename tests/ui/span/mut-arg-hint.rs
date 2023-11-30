trait B {
    fn foo(mut a: &String) {
        a.push_str("bar"); //~ ERROR cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
}

pub fn foo<'a>(mut a: &'a String) {
    a.push_str("foo"); //~ ERROR cannot borrow `*a` as mutable, as it is behind a `&` reference
}

struct A {}

impl A {
    pub fn foo(mut a: &String) {
        a.push_str("foo"); //~ ERROR cannot borrow `*a` as mutable, as it is behind a `&` reference
    }
}

fn main() {
    foo(&"a".to_string());
    A::foo(&"a".to_string());
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
