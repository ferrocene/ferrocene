fn f(_: &mut isize) {}

fn main() {

    let mut x: Box<_> = Box::new(3);

    f(x)    //~ ERROR mismatched types
}

// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
