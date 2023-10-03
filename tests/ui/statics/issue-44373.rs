static FOO: u32 = 50;

fn main() {
    let _val: &'static [&'static u32] = &[&FOO]; //~ ERROR temporary value dropped while borrowed
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
