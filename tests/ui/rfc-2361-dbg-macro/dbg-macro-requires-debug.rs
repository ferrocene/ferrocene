// Test ensuring that `dbg!(expr)` requires the passed type to implement `Debug`.

struct NotDebug;

fn main() {
    let _: NotDebug = dbg!(NotDebug); //~ ERROR `NotDebug` doesn't implement `Debug`
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
