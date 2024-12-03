//@ run-pass

fn f<T:PartialEq + PartialOrd>(_: T) {
}

pub fn main() {
    f(3);
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
