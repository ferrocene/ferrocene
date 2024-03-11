//@ run-pass

pub fn main() {
    let x = [1; 100];
    let mut y = 0;
    for i in &x[..] {
        if y > 10 {
            break;
        }
        y += *i;
    }
    assert_eq!(y, 11);
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
