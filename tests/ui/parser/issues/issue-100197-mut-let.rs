//@ run-rustfix

fn main() {
    mut let _x = 123;
    //~^ ERROR invalid variable declaration
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
