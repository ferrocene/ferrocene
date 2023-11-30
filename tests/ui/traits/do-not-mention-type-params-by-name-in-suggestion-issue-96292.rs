struct Thing<X>(X);

trait Method<T> {
    fn method(self, _: i32) -> T;
}

impl<X> Method<i32> for Thing<X> {
    fn method(self, _: i32) -> i32 { 0 }
}

impl<X> Method<u32> for Thing<X> {
    fn method(self, _: i32) -> u32 { 0 }
}

fn main() {
    let thing = Thing(true);
    thing.method(42);
    //~^ ERROR type annotations needed
    //~| ERROR type annotations needed
}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
