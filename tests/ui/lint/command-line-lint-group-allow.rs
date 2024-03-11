//@ compile-flags: -A bad-style
//@ check-pass

fn main() {
    let _InappropriateCamelCasing = true;
}

// ferrocene-annotations: um_rustc_A
