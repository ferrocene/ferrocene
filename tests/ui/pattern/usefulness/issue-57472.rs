#![crate_type="lib"]
#![deny(unreachable_patterns)]

mod test_struct {
    // Test the exact copy of the minimal example
    // posted in the issue.
    pub struct Punned {
        foo: [u8; 1],
        bar: [u8; 1],
    }

    pub fn test(punned: Punned) {
        match punned {
            Punned { foo: [_], .. } => println!("foo"),
            Punned { bar: [_], .. } => println!("bar"),
            //~^ ERROR unreachable pattern [unreachable_patterns]
        }
    }
}

mod test_union {
    // Test the same thing using a union.
    pub union Punned {
        foo: [u8; 1],
        bar: [u8; 1],
    }

    pub fn test(punned: Punned) {
        match punned {
            Punned { foo: [_] } => println!("foo"),
            Punned { bar: [_] } => println!("bar"),
            //~^ ERROR unreachable pattern [unreachable_patterns]
        }
    }
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Types
