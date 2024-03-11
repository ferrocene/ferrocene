// In this regression test we check that a path pattern referring to a unit variant
// through a type alias is successful in inferring the generic argument.

//@ check-pass

enum Opt<T> {
    N,
    S(T),
}

type OptAlias<T> = Opt<T>;

fn f1(x: OptAlias<u8>) {
    match x {
        OptAlias::N // We previously failed to infer `T` to `u8`.
            => (),
        _ => (),
    }

    match x {
        <
            OptAlias<_> // And we failed to infer this type also.
        >::N => (),
        _ => (),
    }
}

fn main() {}

// ferrocene-annotations: fls_uloyjbaso8pz
// Path Patterns
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
