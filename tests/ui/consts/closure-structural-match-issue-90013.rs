// Regression test for issue 90013.
//@ check-pass

fn main() {
    const { || {} };
}

// ferrocene-annotations: fls_g59pinqkvunq
// Const Blocks
