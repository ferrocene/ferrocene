//@ run-pass
#[derive(Debug, PartialEq, Eq)]
struct IntWrapper(u32);

#[derive(Debug, Ord, PartialOrd, PartialEq, Eq, Hash, Copy, Clone, Default)]
struct HasKeywordField {
    r#struct: u32,
}

struct Generic<r#T>(T);

trait Trait {
    fn r#trait(&self) -> u32;
}
impl Trait for Generic<u32> {
    fn r#trait(&self) -> u32 {
        self.0
    }
}

pub fn main() {
    assert_eq!(IntWrapper(1), r#IntWrapper(1));

    match IntWrapper(2) {
        r#IntWrapper(r#struct) => assert_eq!(2, r#struct),
    }

    assert_eq!("HasKeywordField { struct: 3 }", format!("{:?}", HasKeywordField { r#struct: 3 }));

    assert_eq!(4, Generic(4).0);
    assert_eq!(5, Generic(5).r#trait());
}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
