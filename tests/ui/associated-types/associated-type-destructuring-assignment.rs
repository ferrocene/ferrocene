//@ check-pass

#![feature(more_qualified_paths)]

enum E { V() }

fn main() {
    <E>::V() = E::V(); // OK, destructuring assignment
    <E>::V {} = E::V(); // OK, destructuring assignment
}

// ferrocene-annotations: fls_9beohh5475s2
// Destructuring assignment
