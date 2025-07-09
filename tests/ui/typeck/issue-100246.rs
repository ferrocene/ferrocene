#![recursion_limit = "5"] // To reduce noise

//expect incompatible type error when ambiguous traits are in scope
//and not an overflow error on the span in the main function.

struct Ratio<T>(T);

pub trait Pow {
    fn pow(self) -> Self;
}

impl<'a, T> Pow for &'a Ratio<T>
where
    &'a T: Pow,
{
    fn pow(self) -> Self {
        self
    }
}

fn downcast<'a, W: ?Sized>() -> std::io::Result<&'a W> {
    todo!()
}

struct Other;

fn main() -> std::io::Result<()> {
    let other: Other = downcast()?; //~ ERROR `?` operator has incompatible types
    Ok(())
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
