fn main() {
    for _ in "".to_owned() {}
    //~^ ERROR `String` is not an iterator
    for _ in "" {}
    //~^ ERROR `&str` is not an iterator
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
