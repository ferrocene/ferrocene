// Test that we do not consider parameter types to be sendable without
// an explicit trait bound.

fn foo<T>() {
    is_send::<T>() //~ ERROR E0277
}

fn is_send<T:Send>() {
}

fn main() { }

// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
