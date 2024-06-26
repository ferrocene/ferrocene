// The `?` operator is still not const-evaluatable because it calls `From::from` on the error
// variant.

const fn opt() -> Option<i32> {
    let x = Some(2);
    x?; //~ ERROR `?` is not allowed in a `const fn`
    //~^ ERROR: cannot convert
    //~| ERROR: cannot determine
    None
}

fn main() {}

// ferrocene-annotations: fls_pocsh1neugpc
// Error Propagation Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
