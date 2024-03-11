//@ check-pass
//
// Check that computing the lub works even for empty binders.
fn lt<'a: 'a>() -> &'a () {
    &()
}

fn lt_in_fn<'a: 'a>() -> fn(&'a ()) {
    |_| ()
}

struct Contra<'a>(fn(&'a ()));
fn lt_in_contra<'a: 'a>() -> Contra<'a> {
    Contra(|_| ())
}

fn ok<'a, 'b, 'upper, 'lower>(v: bool)
where
    'upper: 'a,
    'upper: 'b,
    'a: 'lower,
    'b: 'lower,

{
    let _: &'lower () = match v {
        true => lt::<'a>(),
        false => lt::<'b>(),
    };

    // This errored in the past because LUB and GLB always
    // bailed out when encountering binders, even if they were
    // empty.
    let _: fn(&'upper ()) = match v {
        true => lt_in_fn::<'a>(),
        false => lt_in_fn::<'b>(),
    };

    // This was already accepted, as relate didn't encounter any binders.
    let _: Contra<'upper> = match v {
        true => lt_in_contra::<'a>(),
        false => lt_in_contra::<'b>(),
    };
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
