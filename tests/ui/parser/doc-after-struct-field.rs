struct X {
    a: u8 /** document a */,
    //~^ ERROR found a documentation comment that doesn't document anything
    //~| HELP if a comment was intended use `//`
}

struct Y {
    a: u8 /// document a
    //~^ ERROR found a documentation comment that doesn't document anything
    //~| HELP if a comment was intended use `//`
}

fn main() {
    let x = X { a: 1 };
    let y = Y { a: 1 };
}

// ferrocene-annotations: fls_q8l2jza7d9xa
// Comments
