//@ run-pass

// Fast path, main can see the concrete type returned.
fn before() -> impl FnMut(i32) {
    let mut p = Box::new(0);
    move |x| *p = x
}

fn send<T: Send>(_: T) {}

fn main() {
    send(before());
    send(after());
}

// Deferred path, main has to wait until typeck finishes,
// to check if the return type of after is Send.
fn after() -> impl FnMut(i32) {
    let mut p = Box::new(0);
    move |x| *p = x
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
