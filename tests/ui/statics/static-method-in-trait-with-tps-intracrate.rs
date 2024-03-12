//@ run-pass
#![allow(dead_code)]

trait Deserializer {
    fn read_int(&self) -> isize;
}

trait Deserializable<D:Deserializer> {
    fn deserialize(d: &D) -> Self;
}

impl<D:Deserializer> Deserializable<D> for isize {
    fn deserialize(d: &D) -> isize {
        return d.read_int();
    }
}

struct FromThinAir { dummy: () }

impl Deserializer for FromThinAir {
    fn read_int(&self) -> isize { 22 }
}

pub fn main() {
    let d = FromThinAir { dummy: () };
    let i: isize = Deserializable::deserialize(&d);
    assert_eq!(i, 22);
}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
