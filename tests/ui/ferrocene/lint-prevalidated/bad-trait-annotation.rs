#![crate_type = "lib"]
trait Trait {
    #[ferrocene::requires_validation] //~ ERROR cannot be marked
    //~^ HELP use `prevalidated` instead
    fn foo() {}
}
