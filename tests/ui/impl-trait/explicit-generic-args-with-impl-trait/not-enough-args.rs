fn f<T: ?Sized, U: ?Sized>(_: impl AsRef<T>, _: impl AsRef<U>) {}

fn main() {
    f::<[u8]>("a", b"a");
    //~^ ERROR function takes 2 generic arguments but 1 generic argument was supplied
}

// ferrocene-annotations: fls_i7g2n7hfg3ch
// Generic Conformance
