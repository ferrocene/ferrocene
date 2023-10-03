// Test that non-inline modules are not allowed inside blocks.

fn main() {
    mod foo; //~ ERROR cannot declare a non-inline module inside a block
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
