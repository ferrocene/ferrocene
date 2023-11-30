struct R;

impl Drop for R {
    fn drop(&mut self) {
        true //~  ERROR mismatched types
    }
}

fn main() {
}

// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
