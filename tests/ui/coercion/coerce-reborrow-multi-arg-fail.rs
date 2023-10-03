fn test<T>(_a: T, _b: T) {}

fn main() {
    test(&mut 7, &7);
    //~^ mismatched types
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
