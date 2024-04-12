//@ run-rustfix
#[allow(unused_mut)]
fn test() {
    let w: &mut [isize];
    w[5] = 0; //~ ERROR [E0381]

    let mut w: &mut [isize];
    w[5] = 0; //~ ERROR [E0381]
}

fn main() { test(); }

//
// ferrocene-annotations: fls_sxcr4aa098i6
// Array and Slice Indexing Expressions
