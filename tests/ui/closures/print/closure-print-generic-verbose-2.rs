//@ compile-flags: -Zverbose-internals

mod mod1 {
    pub fn f<T: std::fmt::Display>(t: T)
    {
        let x = 20;

        let c = || println!("{} {}", t, x);
        let c1 : () = c;
        //~^ ERROR mismatched types
    }
}

fn main() {
    mod1::f(5i32);
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_xd2oxlebhs14
// Closure Type
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
