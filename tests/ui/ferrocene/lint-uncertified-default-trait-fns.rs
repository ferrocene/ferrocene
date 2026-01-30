// Don't use anything from core here.
// If you do, it will break once we start annotating more of core.
// You can still use traits as long as the impls use custom types.

#![crate_type = "lib"]
#![deny(ferrocene::uncertified)] //~ NOTE lint level

trait UncertifiedDefault {
    fn foo(&self) {} //~ NOTE unvalidated
}

trait CertifiedDefault {
    #[ferrocene::prevalidated]
    fn bar(&self) {}
}

struct OverridesDefault;
impl UncertifiedDefault for OverridesDefault {
    #[ferrocene::prevalidated]
    fn foo(&self) {}
}
impl CertifiedDefault for OverridesDefault {
    fn bar(&self) {} //~ NOTE unvalidated
}

struct UsesDefault;
impl UncertifiedDefault for UsesDefault {}
impl CertifiedDefault for UsesDefault {}

#[ferrocene::prevalidated] //~ NOTE marked
fn test() { //~ NOTE is validated
    OverridesDefault.foo(); // ok
    OverridesDefault.bar(); //~ ERROR unvalidated
    UsesDefault.foo(); //~ ERROR unvalidated
    UsesDefault.bar(); // ok
}
