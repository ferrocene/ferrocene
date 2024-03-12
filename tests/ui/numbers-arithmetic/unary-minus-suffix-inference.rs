//@ run-pass

pub fn main() {
    let a = 1;
    let a_neg: i8 = -a;
    println!("{}", a_neg);

    let b = 1;
    let b_neg: i16 = -b;
    println!("{}", b_neg);

    let c = 1;
    let c_neg: i32 = -c;
    println!("{}", c_neg);

    let d = 1;
    let d_neg: i64 = -d;
    println!("{}", d_neg);

    let e = 1;
    let e_neg: isize = -e;
    println!("{}", e_neg);
}

// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_wrecura8u5ar
// Negation Expression
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
