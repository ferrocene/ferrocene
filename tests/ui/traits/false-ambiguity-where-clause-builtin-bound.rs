//@ run-pass
// Test that we do not error out because of a (False) ambiguity
// between the builtin rules for Sized and the where clause. Issue
// #20959.


fn foo<K>(x: Option<K>)
    where Option<K> : Sized
{
    let _y = x;
}

fn main() {
    foo(Some(22));
}
// ferrocene-annotations: fls_ikfvbeewame7
// Subtyping and Variance
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
