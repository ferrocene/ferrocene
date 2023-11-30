pub fn bar<F: Fn()>(_f: F) {} //~ NOTE change this to accept `FnMut` instead of `Fn`

pub fn foo() {
    let mut x = 0;
    bar(move || x = 1);
    //~^ ERROR cannot assign to `x`, as it is a captured variable in a `Fn` closure
    //~| NOTE cannot assign
    //~| NOTE expects `Fn` instead of `FnMut`
    //~| NOTE in this closure
}

fn main() {}

// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
