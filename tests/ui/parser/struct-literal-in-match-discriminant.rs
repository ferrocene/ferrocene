<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
struct Foo {
    x: isize,
}

fn main() {
    match Foo { //~ ERROR struct literals are not allowed here
        x: 3
    } {
        Foo {
            x: x
        } => {}
    }
}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
