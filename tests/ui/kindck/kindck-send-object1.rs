// Test which object types are considered sendable. This test
// is broken into two parts because some errors occur in distinct
// phases in the compiler. See kindck-send-object2.rs as well!

fn assert_send<T:Send+'static>() { }
trait Dummy { }

// careful with object types, who knows what they close over...
fn test51<'a>() {
    assert_send::<&'a dyn Dummy>();
    //~^ ERROR `&'a (dyn Dummy + 'a)` cannot be sent between threads safely [E0277]
}
fn test52<'a>() {
    assert_send::<&'a (dyn Dummy + Sync)>();
    //~^ ERROR: lifetime may not live long enough
}

// ...unless they are properly bounded
fn test60() {
    assert_send::<&'static (dyn Dummy + Sync)>();
}
fn test61() {
    assert_send::<Box<dyn Dummy + Send>>();
}

// closure and object types can have lifetime bounds which make
// them not ok
fn test_71<'a>() {
    assert_send::<Box<dyn Dummy + 'a>>();
    //~^ ERROR `(dyn Dummy + 'a)` cannot be sent between threads safely
}

fn main() { }

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
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Type
