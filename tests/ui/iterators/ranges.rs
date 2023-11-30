fn main() {
    for _ in ..10 {}
    //~^ ERROR E0277
    for _ in ..=10 {}
    //~^ ERROR E0277
    for _ in 0..10 {}
    for _ in 0..=10 {}
    for _ in 0.. {}
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
