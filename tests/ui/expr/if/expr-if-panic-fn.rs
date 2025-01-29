//@ run-fail
//@ error-pattern:explicit panic
//@ needs-subprocess

fn f() -> ! {
    panic!()
}

fn g() -> isize {
    let x = if true {
        f()
    } else {
        10
    };
    return x;
}

fn main() {
    g();
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
