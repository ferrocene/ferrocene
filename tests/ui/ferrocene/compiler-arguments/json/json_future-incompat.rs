//@ check-pass
//@ compile-flags: --error-format=json --json=future-incompat
//@ dont-require-annotations: WARN

// From compiler/rustc_lint_defs/src/builtin.rs

struct S;

impl S {
    fn late(self, _: &u8, _: &u8) {}
}

fn main() {
    S.late::<'static>(&0, &0);
}

// ferrocene-annotations: um_rustc_json
// ferrocene-annotations: um_rustc_error_format
