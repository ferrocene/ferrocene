// Test ensuring that `dbg!(expr)` will take ownership of the argument.

#[derive(Debug)]
struct NoCopy(usize);

fn main() {
    let a = NoCopy(0);
    let _ = dbg!(a);
    let _ = dbg!(a); //~ ERROR use of moved value
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
