// #84044: This used to ICE.

fn main() {
    let f = || {};
    drop(&mut f); //~ ERROR cannot borrow `f` as mutable, as it is not declared as mutable
}

// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
