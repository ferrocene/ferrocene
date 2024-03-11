//@ run-fail
//@ error-pattern:oops
//@ ignore-emscripten no processes

fn main() {
    let func = || -> ! {
        panic!("oops");
    };
    func();
}

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
