fn main() {
    assert_eq!(1, 2) //~ ERROR: expected `;`
    assert_eq!(3, 4) //~ ERROR: expected `;`
    println!("hello");
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
