fn main() {
    let b = 2;
    let _: fn(usize) -> usize = match true {
        true => |a| a + 1,
        false => |a| a - b,
        //~^ ERROR `match` arms have incompatible types
    };
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
