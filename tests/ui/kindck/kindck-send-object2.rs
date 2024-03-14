// Continue kindck-send-object1.rs.

fn assert_send<T:Send>() { }
trait Dummy { }

fn test50() {
    assert_send::<&'static dyn Dummy>();
    //~^ ERROR `&'static (dyn Dummy + 'static)` cannot be sent between threads safely [E0277]
}

fn test53() {
    assert_send::<Box<dyn Dummy>>();
    //~^ ERROR `dyn Dummy` cannot be sent between threads safely
}

// ...unless they are properly bounded
fn test60() {
    assert_send::<&'static (dyn Dummy + Sync)>();
}
fn test61() {
    assert_send::<Box<dyn Dummy + Send>>();
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
