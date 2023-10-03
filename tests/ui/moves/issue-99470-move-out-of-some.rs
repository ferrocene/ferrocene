fn main() {
    let x: &Option<Box<i32>> = &Some(Box::new(0));

    match x {
    //~^ ERROR cannot move out of `x` as enum variant `Some` which is behind a shared reference
        &Some(_y) => (),
        &None => (),
    }
}

// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
