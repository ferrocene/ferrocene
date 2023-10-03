fn main() {
}

fn foo<T>(x: T, y: T) {
    let z = x + y; //~ ERROR cannot add `T` to `T`
}

fn bar<T>(x: T) {
    x += x; //~ ERROR binary assignment operation `+=` cannot be applied to type `T`
}

fn baz<T>(x: T) {
    let y = -x; //~ ERROR cannot apply unary operator `-` to type `T`
    let y = !x; //~ ERROR cannot apply unary operator `!` to type `T`
    let y = *x; //~ ERROR type `T` cannot be dereferenced
}

// ferrocene-annotations: fls_wrecura8u5ar
// Negation Expression
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression
// ferrocene-annotations: fls_290jmzfh7x4e
// Compound Assignment Expressions
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
