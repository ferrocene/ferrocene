// Test that binary operators consume their arguments

use std::ops::{Add, Sub, Mul, Div, Rem, BitAnd, BitXor, BitOr, Shl, Shr};

fn add<A: Add<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs + rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn sub<A: Sub<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs - rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn mul<A: Mul<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs * rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn div<A: Div<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs / rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn rem<A: Rem<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs % rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn bitand<A: BitAnd<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs & rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn bitor<A: BitOr<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs | rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn bitxor<A: BitXor<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs ^ rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn shl<A: Shl<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs << rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn shr<A: Shr<B, Output=()>, B>(lhs: A, rhs: B) {
    lhs >> rhs;
    drop(lhs);  //~ ERROR use of moved value: `lhs`
    drop(rhs);  //~ ERROR use of moved value: `rhs`
}

fn main() {}

// ferrocene-annotations: fls_abp6tjbz8tpn
// Bit Expressions
//
// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
