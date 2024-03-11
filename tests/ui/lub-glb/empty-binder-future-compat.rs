//@ check-pass
fn lt_in_fn_fn<'a: 'a>() -> fn(fn(&'a ())) {
    |_| ()
}


fn foo<'a, 'b, 'lower>(v: bool)
where
    'a: 'lower,
    'b: 'lower,
{
        // if we infer `x` to be higher ranked in the future,
        // this would cause a type error.
        let x = match v {
            true => lt_in_fn_fn::<'a>(),
            false => lt_in_fn_fn::<'b>(),
        };

        let _: fn(fn(&'lower())) = x;
}

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetime
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
