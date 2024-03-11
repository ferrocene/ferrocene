//@ run-pass

pub fn main() {
    let i: isize = if false { panic!() } else { 5 };
    println!("{}", i);
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
