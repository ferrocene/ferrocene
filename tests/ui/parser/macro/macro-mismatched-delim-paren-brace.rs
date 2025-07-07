fn main() {
    foo! (
        bar, "baz", 1, 2.0
    } //~ ERROR mismatched closing delimiter
} //~ ERROR unexpected closing delimiter: `}`

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
