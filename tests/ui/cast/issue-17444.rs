enum Test {
    Foo = 0
}

fn main() {
    let _x = Test::Foo as *const isize;
    //~^ ERROR casting `Test` as `*const isize` is invalid
}

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Type
