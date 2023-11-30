fn main() {
    let x: Option<&[u8]> = Some("foo").map(std::mem::transmute);
    //~^ ERROR E0277
}

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
