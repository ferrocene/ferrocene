#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(non_snake_case)]
#![deny(unreachable_patterns)]

#[derive(Clone, Copy)]
enum Enum {
    Var1,
    Var2,
}

fn main() {
    use Enum::*;
    let s = Var1;
    match s {
        Var1 => (),
        Var3 => (),
        Var2 => (),
        //~^ ERROR unreachable pattern
    };
    match &s {
        &Var1 => (),
        &Var3 => (),
        &Var2 => (),
        //~^ ERROR unreachable pattern
    };
    let t = (Var1, Var1);
    match t {
        (Var1, b) => (),
        (c, d) => (),
        anything => ()
        //~^ ERROR unreachable pattern
    };
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
