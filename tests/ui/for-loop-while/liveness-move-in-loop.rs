//@ run-pass
#![allow(dead_code)]


fn take(x: isize) -> isize {x}

fn the_loop() {
    let mut list = Vec::new();
    loop {
        let x = 5;
        if x > 3 {
            list.push(take(x));
        } else {
            break;
        }
    }
}

pub fn main() {}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
