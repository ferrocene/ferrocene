pub fn main() {


    let x: Box<_> = Box::new(1);

    let v = (1, 2);

    match v {
        (1, _) if take(x) => (),
        (_, 2) if take(x) => (), //~ ERROR use of moved value: `x`
        _ => (),
    }
}

fn take<T>(_: T) -> bool { false }

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
