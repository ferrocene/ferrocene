//@ compile-flags: -W bad-style
//@ check-pass

fn main() {
    let _InappropriateCamelCasing = true;
    //~^ WARNING should have a snake case name
}

// ferrocene-annotations: um_rustc_W
