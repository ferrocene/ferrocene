//@ run-pass

fn main() {
    'outer: loop {
        let _: i32 = loop { break 'outer };
    }
    'outer2: loop {
        let _: i32 = loop { loop { break 'outer2 } };
    }
}

// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
