//@ run-fail
//@ error-pattern:quux
//@ ignore-emscripten no processes

fn my_err(s: String) -> ! {
    println!("{}", s);
    panic!("quux");
}

fn main() {
    if my_err("bye".to_string()) {
    }
}

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
