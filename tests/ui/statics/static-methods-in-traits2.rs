//@ run-pass

pub trait Number: NumConv {
    fn from<T:Number>(n: T) -> Self;
}

impl Number for f64 {
    fn from<T:Number>(n: T) -> f64 { n.to_float() }
}

pub trait NumConv {
    fn to_float(&self) -> f64;
}

impl NumConv for f64 {
    fn to_float(&self) -> f64 { *self }
}

pub fn main() {
    let _: f64 = Number::from(0.0f64);
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
