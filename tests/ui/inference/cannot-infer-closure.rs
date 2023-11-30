fn main() {
    let x = |a: (), b: ()| {
        Err(a)?;
        Ok(b)
        //~^ ERROR type annotations needed
    };
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_pocsh1neugpc
// Error Propagation Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
