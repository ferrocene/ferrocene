trait Trait {}
impl Trait for () {}

fn bad_echo<T>(_t: T) -> T {
    "this should not suggest impl Trait" //~ ERROR mismatched types
}

fn bad_echo_2<T: Trait>(_t: T) -> T {
    "this will not suggest it, because that would probably be wrong" //~ ERROR mismatched types
}

fn other_bounds_bad<T>() -> T
where
    T: Send,
    Option<T>: Send,
{
    "don't suggest this, because Option<T> places additional constraints" //~ ERROR mismatched types
}

// FIXME: implement this check
trait GenericTrait<T> {}

fn used_in_trait<T>() -> T
where
    T: Send,
    (): GenericTrait<T>,
{
    "don't suggest this, because the generic param is used in the bound." //~ ERROR mismatched types
}

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
