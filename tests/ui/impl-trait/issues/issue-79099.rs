//@ edition: 2015
struct Bug {
    V1: [(); {
        let f: impl core::future::Future<Output = u8> = async { 1 };
        //~^ `impl Trait` is not allowed in the type of variable bindings
        //~| expected identifier
        1
    }],
}

fn main() {}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
