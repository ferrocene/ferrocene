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

fn covariance<'a, 'b, 'upper>(v: bool)
where
    'upper: 'a,
    'upper: 'b,

{
    let _: &'upper () = match v {
        //~^ ERROR lifetime may not live long enough
        //~| ERROR lifetime may not live long enough
        true => lt::<'a>(),
        false => lt::<'b>(),
    };
}

fn contra_fn<'a, 'b, 'lower>(v: bool)
where
    'a: 'lower,
    'b: 'lower,

{

    let _: fn(&'lower ()) = match v {
        //~^ ERROR lifetime may not live long enough
        true => lt_in_fn::<'a>(),
        false => lt_in_fn::<'b>(),
    };
}

fn contra_struct<'a, 'b, 'lower>(v: bool)
where
    'a: 'lower,
    'b: 'lower,

{
    let _: Contra<'lower> = match v {
        //~^ ERROR lifetime may not live long enough
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
