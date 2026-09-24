// Test that we can't pass other types for !

fn foo(x: !) -> ! {
    x
}

fn main() {
    foo("wow"); //~ ERROR mismatched types
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
