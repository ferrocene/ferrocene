fn f<'a: 'static>(_: &'a i32) {} //~WARN unnecessary lifetime parameter `'a`

fn main() {
    let x = 0;
    f(&x); //~ERROR does not live long enough
}

// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
