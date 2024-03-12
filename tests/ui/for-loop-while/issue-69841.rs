// This is a regression test for issue rust-lang/rust#69841, which exposed an
// LLVM bug which needed a fix to be backported.

//@ run-pass

fn main() {
    let buffer = [49u8, 10];
    let mut a : u64 = 0;
    'read: loop {
        for c in &buffer {
            match c {
                48..=57 => {
                    a*= 10;
                    a+= *c as u64 - 48;
                }
                10 => {
                    break 'read;
                }
                _ => {
                    unsafe { std::hint::unreachable_unchecked() };
                }
            }
        }
    }
    if a == 1 {
        println!("What did you expect?");
    } else {
        panic!("this should be unreachable.");
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
