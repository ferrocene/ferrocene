// Check that inherent impls cannot be unsafe.

struct SomeStruct;

unsafe impl SomeStruct { //~ ERROR inherent impls cannot be unsafe
    fn foo(self) { }
}

fn main() { }

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
