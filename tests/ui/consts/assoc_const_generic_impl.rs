//@ build-fail

trait ZeroSized: Sized {
    const I_AM_ZERO_SIZED: ();
    fn requires_zero_size(self);
}

impl<T: Sized> ZeroSized for T {
    const I_AM_ZERO_SIZED: ()  = [()][std::mem::size_of::<Self>()]; //~ ERROR evaluation of `<u32 as ZeroSized>::I_AM_ZERO_SIZED` failed
    fn requires_zero_size(self) {
        Self::I_AM_ZERO_SIZED;
        println!("requires_zero_size called");
    }
}

fn main() {
    ().requires_zero_size();
    42_u32.requires_zero_size();
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated items

// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation conformance

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
