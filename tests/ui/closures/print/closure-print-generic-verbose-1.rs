//@ compile-flags: -Zverbose-internals

fn to_fn_once<F:FnOnce()>(f: F) -> F { f }

fn f<T: std::fmt::Display>(y: T) {
    struct Foo<U: std::fmt::Display> {
        x: U
    };

    let foo =  Foo{ x: "x" };

    let c = to_fn_once(move|| {
        println!("{} {}", foo.x, y);
    });

    c();
    c();
    //~^ ERROR use of moved value
}


fn main() {
    f("S");
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
