fn main() {
    {
        if (foo) => {} //~ ERROR expected `{`, found `=>`
    }
    {
        if (foo)
            bar; //~ ERROR expected `{`, found `bar`
    }
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
