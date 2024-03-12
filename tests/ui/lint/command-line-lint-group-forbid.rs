//@ compile-flags: -F bad-style

fn main() {
    let _InappropriateCamelCasing = true; //~ ERROR should have a snake
}

// ferrocene-annotations: um_rustc_F
