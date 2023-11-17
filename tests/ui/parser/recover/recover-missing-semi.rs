fn main() {
    let _: usize = ()
    //~^ ERROR mismatched types
    //~| ERROR expected `;`
    let _ = 3;
}

fn foo() -> usize {
    let _: usize = ()
    //~^ ERROR mismatched types
    //~| ERROR expected `;`
    return 3;
}

// ferrocene-annotations: fls_yivm43r5wnp1
// Let Statements
