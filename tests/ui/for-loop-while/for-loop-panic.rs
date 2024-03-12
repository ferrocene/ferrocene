//@ run-pass


pub fn main() { let x: Vec<isize> = Vec::new(); for _ in &x { panic!("moop"); } }

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
