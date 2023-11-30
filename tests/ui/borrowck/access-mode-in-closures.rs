struct S(Vec<isize>);

fn unpack<F>(_unpack: F) where F: FnOnce(&S) -> Vec<isize> {}

fn main() {
    let _foo = unpack(|s| {
        // Test that `s` is moved here.
        match *s { S(v) => v } //~ ERROR cannot move out
    });
}

//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
