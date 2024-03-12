//@ run-pass
// Issue #12512.

//@ pretty-expanded FIXME #23616

fn main() {
    let mut foo = Vec::new();
    #[allow(unused_labels)]
    'foo: for i in &[1, 2, 3] {
        foo.push(*i);
    }
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
