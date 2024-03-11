//@ compile-flags: -D bad-style

fn main() {
    let _InappropriateCamelCasing = true; //~ ERROR should have a snake
}

// ferrocene-annotations: um_rustc_D
