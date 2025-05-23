//@ dont-require-annotations: NOTE

fn needs_fn<F>(x: F) where F: Fn(isize) -> isize {}

fn main() {
    let _: () = Box::new(|_: isize| {}) as Box<dyn FnOnce(isize)>;
    //~^ ERROR mismatched types
    //~| NOTE expected unit type `()`
    //~| NOTE found struct `Box<dyn FnOnce(isize)>`
    let _: () = Box::new(|_: isize, isize| {}) as Box<dyn Fn(isize, isize)>;
    //~^ ERROR mismatched types
    //~| NOTE expected unit type `()`
    //~| NOTE found struct `Box<dyn Fn(isize, isize)>`
    let _: () = Box::new(|| -> isize { unimplemented!() }) as Box<dyn FnMut() -> isize>;
    //~^ ERROR mismatched types
    //~| NOTE expected unit type `()`
    //~| NOTE found struct `Box<dyn FnMut() -> isize>`

    needs_fn(1);
    //~^ ERROR expected a `Fn(isize)` closure, found `{integer}`
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
