use std::rc::Rc;

fn foo(_x: Rc<usize>) {}

fn bar<F:FnOnce() + Send>(_: F) { }

fn main() {
    let x = Rc::new(3);
    bar(move|| foo(x));
    //~^ ERROR `Rc<usize>` cannot be sent between threads safely
}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
