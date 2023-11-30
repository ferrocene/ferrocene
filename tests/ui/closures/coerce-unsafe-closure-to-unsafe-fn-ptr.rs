// revisions: mir thir
// [thir]compile-flags: -Z thir-unsafeck

fn main() {
    let _: unsafe fn() = || { ::std::pin::Pin::new_unchecked(&0_u8); };
    //~^ ERROR E0133
    let _: unsafe fn() = || unsafe { ::std::pin::Pin::new_unchecked(&0_u8); }; // OK
}

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
