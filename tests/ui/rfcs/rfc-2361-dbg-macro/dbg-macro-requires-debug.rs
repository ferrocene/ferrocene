// Test ensuring that `dbg!(expr)` requires the passed type to implement `Debug`.
//
// `dbg!` shouldn't tell the user about format literal syntax; the user didn't write one.
//@ forbid-output: cannot be formatted using

struct NotDebug;

fn main() {
    let _: NotDebug = dbg!(NotDebug); //~ ERROR `NotDebug` doesn't implement `Debug`
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
