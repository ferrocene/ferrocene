// Checks that const parameters cannot be shadowed with fresh bindings
// even in syntactically unambiguous contexts. See
// https://github.com/rust-lang/rust/issues/33118#issuecomment-233962221

fn foo<const N: i32>(i: i32) -> bool {
    match i {
        N @ _ => true,
        //~^ ERROR: match bindings cannot shadow const parameters [E0530]
    }
}

fn bar<const N: i32>(i: i32) -> bool {
    let N @ _ = 0;
    //~^ ERROR: let bindings cannot shadow const parameters [E0530]
    match i {
        N @ _ => true,
    }
}

fn main() {}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
