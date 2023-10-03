fn main() {
    let x = -5;
    if x<-1 { //~ ERROR unexpected token: `<-`
        println!("ok");
    }
}

// ferrocene-annotations: fls_wrecura8u5ar
// Negation Expression
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
