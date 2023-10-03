struct A<T>(T);

impl A<bool> {
    const B: A<u8> = Self(0);
    //~^ ERROR mismatched types
    //~| ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
