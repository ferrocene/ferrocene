fn f() {
    'l: loop {
        fn g() {
            loop {
                break 'l; //~ ERROR use of unreachable label
            }
        }
    }

    loop { 'w: while break 'w { } }
}

fn main() {}

// ferrocene-annotations: fls_rr908hgunja7
// Loop Expressions
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
