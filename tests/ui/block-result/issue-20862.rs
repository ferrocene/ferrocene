fn foo(x: i32) {
    |y| x + y
//~^ ERROR: mismatched types
}

fn main() {
    let x = foo(5)(2);
//~^ ERROR: expected function, found `()`
}

// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
