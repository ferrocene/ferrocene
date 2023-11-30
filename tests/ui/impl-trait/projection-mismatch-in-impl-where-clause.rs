pub trait Super {
    type Assoc;
}

impl Super for () {
    type Assoc = u8;
}

pub trait Test {}

impl<T> Test for T where T: Super<Assoc = ()> {}

fn test() -> impl Test {
    //~^ERROR type mismatch resolving `<() as Super>::Assoc == ()`
    ()
}

fn main() {
    let a = test();
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
