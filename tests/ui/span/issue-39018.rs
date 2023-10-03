pub fn main() {
    let x = "Hello " + "World!";
    //~^ ERROR cannot add

    // Make sure that the span outputs a warning
    // for not having an implementation for std::ops::Add
    // that won't output for the above string concatenation
    let y = World::Hello + World::Goodbye;
    //~^ ERROR cannot add

    let x = "Hello " + "World!".to_owned();
    //~^ ERROR cannot add
}

enum World {
    Hello,
    Goodbye,
}

fn foo() {
    let a = String::new();
    let b = String::new();
    let c = "";
    let d = "";
    let e = &a;
    let _ = &a + &b; //~ ERROR cannot add
    let _ = &a + b; //~ ERROR cannot add
    let _ = a + &b; // ok
    let _ = a + b; //~ ERROR mismatched types
    let _ = e + b; //~ ERROR cannot add
    let _ = e + &b; //~ ERROR cannot add
    let _ = e + d; //~ ERROR cannot add
    let _ = e + &d; //~ ERROR cannot add
    let _ = &c + &d; //~ ERROR cannot add
    let _ = &c + d; //~ ERROR cannot add
    let _ = c + &d; //~ ERROR cannot add
    let _ = c + d; //~ ERROR cannot add
}

// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
