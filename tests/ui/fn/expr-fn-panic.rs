//@ run-fail
//@ error-pattern:explicit panic
//@ needs-subprocess

fn f() -> ! {
    panic!()
}

fn main() {
    f();
}

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
