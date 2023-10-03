fn loop_ending() -> i32 {
    loop {
        if false { break; } //~ ERROR mismatched types
        return 42;
    }
}

fn main() {}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
