//@ run-rustfix

fn main() {
    const let _FOO: i32 = 123;
    //~^ ERROR const` and `let` are mutually exclusive
    let const _BAR: i32 = 123;
    //~^ ERROR `const` and `let` are mutually exclusive
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
