fn main() {
    loop {
        let _: i32 = loop { break }; //~ ERROR mismatched types
    }
    loop {
        let _: i32 = 'inner: loop { break 'inner }; //~ ERROR mismatched types
    }
    loop {
        let _: i32 = 'inner2: loop { loop { break 'inner2 } }; //~ ERROR mismatched types
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
