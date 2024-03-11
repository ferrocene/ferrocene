//@ check-pass

union URes<R: Copy> {
    uninit: (),
    init: R,
}

struct Params<F, R: Copy> {
    function: F,
    result: URes<R>,
}

unsafe extern "C" fn do_call<F, R>(params: *mut Params<F, R>)
where
    R: Copy,
    F: Fn() -> R,
{
    (*params).result.init = ((*params).function)();
}

fn main() {}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
//
// ferrocene-annotations: fls_g0uyl7qw4c7w
// Parenthesized Expressions
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
