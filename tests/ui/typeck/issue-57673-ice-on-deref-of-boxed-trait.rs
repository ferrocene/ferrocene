//extern crate has_assoc_type;

//fn ice(x: Box<dyn has_assoc_type::Foo<Assoc=()>>) {
fn ice(x: Box<dyn Iterator<Item=()>>) {
    *x //~ ERROR mismatched types [E0308]
}
fn main() {}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
