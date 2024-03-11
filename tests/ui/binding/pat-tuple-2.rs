//@ run-pass
fn tuple() {
    let x = (1,);
    match x {
        (2, ..) => panic!(),
        (..) => ()
    }
}

fn tuple_struct() {
    struct S(u8);

    let x = S(1);
    match x {
        S(2, ..) => panic!(),
        S(..) => ()
    }
}

fn main() {
    tuple();
    tuple_struct();
}

// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest patterns
//
// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple pattern matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple patterns
//
// ferrocene-annotations: fls_eexupzdsu7f
// Tuple struct pattern matching
//
// ferrocene-annotations: fls_vlrto778v49m
// Tuple struct patterns
