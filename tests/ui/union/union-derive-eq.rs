//@ revisions: current next
//@[next] compile-flags: -Znext-solver
//@ ignore-compare-mode-next-solver (explicit revisions)

#[derive(Eq)] // OK
union U1 {
    a: u8,
}

impl PartialEq for U1 {
    fn eq(&self, rhs: &Self) -> bool {
        true
    }
}

#[derive(PartialEq, Copy, Clone)]
struct PartialEqNotEq;

#[derive(Eq)]
union U2 {
    a: PartialEqNotEq, //~ ERROR the trait bound `PartialEqNotEq: Eq` is not satisfied
}

impl PartialEq for U2 {
    fn eq(&self, rhs: &Self) -> bool {
        true
    }
}

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
