enum Foo {
    Bar(Bar),
}
struct Bar {
    x: i32,
}

fn a(f: Foo) {
    match f {
        Bar { x } => {
            //~^ ERROR mismatched types
            //~| HELP try wrapping
        }
    }
}

struct S;

fn b(s: Option<S>) {
    match s {
        S => {
            //~^ ERROR mismatched types
            //~| HELP try wrapping
            //~| HELP introduce a new binding instead
        }
        _ => {}
    }
}

fn c(s: Result<S, S>) {
    match s {
        S => {
            //~^ ERROR mismatched types
            //~| HELP try wrapping
            //~| HELP introduce a new binding instead
        }
        _ => {}
    }
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jm6l7b90h6wa
// Pattern Matching
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
