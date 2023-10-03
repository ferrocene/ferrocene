fn main() {
    let x = Some(1);
    let y = x.or_else(4);
    //~^ ERROR expected a `FnOnce<()>` closure, found `{integer}`
}

// ferrocene-annotations: fls_dw33yt5g6m0k
// Type Coercion
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
