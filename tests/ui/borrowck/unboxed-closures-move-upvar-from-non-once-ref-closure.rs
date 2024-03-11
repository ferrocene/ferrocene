//@ run-rustfix
// Test that a by-ref `FnMut` closure gets an error when it tries to
// consume a value.

fn call<F>(f: F) where F : Fn() {
    f();
}

fn main() {
    let y = vec![format!("World")];
    call(|| {
        y.into_iter();
        //~^ ERROR cannot move out of `y`, a captured variable in an `Fn` closure
    });
}

//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_v5x85lt5ulva
// References
