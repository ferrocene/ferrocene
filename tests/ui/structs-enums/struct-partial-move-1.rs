//@ run-pass
#[derive(PartialEq, Debug)]
pub struct Partial<T> { x: T, y: T }

#[derive(PartialEq, Debug)]
struct S { val: isize }
impl S { fn new(v: isize) -> S { S { val: v } } }
impl Drop for S { fn drop(&mut self) { } }

pub fn f<T, F>((b1, b2): (T, T), mut f: F) -> Partial<T> where F: FnMut(T) -> T {
    let p = Partial { x: b1, y: b2 };

    // Move of `p` is legal even though we are also moving `p.y`; the
    // `..p` moves all fields *except* `p.y` in this context.
    Partial { y: f(p.y), ..p }
}

pub fn main() {
    let p = f((S::new(3), S::new(4)), |S { val: z }| S::new(z+1));
    assert_eq!(p, Partial { x: S::new(3), y: S::new(5) });
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_77scxuomlbgs
// Passing Conventions
//
// ferrocene-annotations: fls_7dbd5t2750ce
// Struct Patterns
//
// ferrocene-annotations: fls_nruvg0es3kx7
// Record Struct Patterns
