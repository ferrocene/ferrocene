//@ run-fail
//@ error-pattern:oops
//@ needs-subprocess

fn main() {
    let func = || -> ! {
        panic!("oops");
    };
    func();
}

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
