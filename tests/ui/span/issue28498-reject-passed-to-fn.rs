// Demonstrate that a type param in negative position causes dropck to reject code
// that might indirectly access previously dropped value.
//
// Compare with run-pass/issue28498-ugeh-with-passed-to-fn.rs

#[derive(Debug)]
struct ScribbleOnDrop(String);

impl Drop for ScribbleOnDrop {
    fn drop(&mut self) {
        self.0 = format!("DROPPED");
    }
}

struct Foo<T>(u32, T, Box<for <'r> fn(&'r T) -> String>);

impl<T> Drop for Foo<T> {
    fn drop(&mut self) {
        // Use of `may_dangle` is unsound, because we pass `T` to the callback in `self.2`
        // below, and thus potentially read from borrowed data.
        println!("Dropping Foo({}, {})", self.0, (self.2)(&self.1));
    }
}

fn callback(s: & &ScribbleOnDrop) -> String { format!("{:?}", s) }

fn main() {
    let (last_dropped, foo0);
    let (foo1, first_dropped);

    last_dropped = ScribbleOnDrop(format!("last"));
    first_dropped = ScribbleOnDrop(format!("first"));
    foo0 = Foo(0, &last_dropped, Box::new(callback)); // OK
    foo1 = Foo(1, &first_dropped, Box::new(callback));
    //~^ ERROR `first_dropped` does not live long enough

    println!("foo0.1: {:?} foo1.1: {:?}", foo0.1, foo1.1);
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_4jiw35pan7vn
// Destruction
//
// ferrocene-annotations: fls_u2mzjgiwbkz0
// Destructors
//
// ferrocene-annotations: fls_rm4ncoopcdvj
// Drop Scopes
//
// ferrocene-annotations: fls_afafmafz4hf2
// Drop Order
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
