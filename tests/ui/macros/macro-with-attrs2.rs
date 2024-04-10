//@ run-pass

#[cfg(FALSE)]
macro_rules! foo { () => (1) }

#[cfg(not(FALSE))]
macro_rules! foo { () => (2) }

pub fn main() {
    assert_eq!(foo!(), 2);
}

// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
//
// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
