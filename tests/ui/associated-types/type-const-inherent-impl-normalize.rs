struct S<const N: usize>;
impl<const N: usize> S<N> {
    #[type_const]
    //~^ ERROR: the `#[type_const]` attribute is an experimental feature
    const LEN: usize = 1;
    fn arr() {
        [8; Self::LEN]
        //~^ WARN: cannot use constants which depend on generic parameters in types
        //~| WARN: this was previously accepted by the compiler but is being phased out
        //~| WARN: cannot use constants which depend on generic parameters in types
        //~| WARN: this was previously accepted by the compiler but is being phased out
        //~| ERROR: mismatched types
    }
}

pub fn main() {}
