//@ run-fail
//@ error-pattern:squirrelcupcake
//@ ignore-emscripten no processes

fn cmp() -> isize {
    match (Some('a'), None::<char>) {
        (Some(_), _) => {
            panic!("squirrelcupcake");
        }
        (_, Some(_)) => {
            panic!();
        }
        _ => {
            panic!("wat");
        }
    }
}

fn main() {
    println!("{}", cmp());
}

// ferrocene-annotations: fls_yc4xm4hrfyw7
// Wildcard Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Wildcard Patterns
