//@ run-pass

mod m {
    pub fn f<T>(_: T, _: ()) { }
    pub fn g<T>(_: T, _: ()) { }
}

const BAR: () = ();
struct Data;
use m::f;

fn main() {
    const BAR2: () = ();
    struct Data2;
    use m::g;

    f(Data, BAR);
    g(Data2, BAR2);
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
