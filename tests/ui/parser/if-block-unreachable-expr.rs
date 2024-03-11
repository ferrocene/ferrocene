//@ check-pass

// This regressed from 1.20 -> 1.21 -- the condition is unreachable,
// but it's still an expression, and should parse fine.

fn main() {
    if { if true { return; } else { return; }; } {}
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
