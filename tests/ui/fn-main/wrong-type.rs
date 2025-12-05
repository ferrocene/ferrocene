<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
struct S {
    x: isize,
    y: isize,
}

fn main(foo: S) {
//~^ ERROR: `main` function has wrong type [E0580]
}

// ferrocene-annotations: fls_8jb3sjqamdpu
// Program Entry Point
