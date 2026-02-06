#![crate_type = "lib"]
#![deny(ferrocene::uncertified)] //~ NOTE lint level

struct Unvalidated;

trait SuperTrait { fn foo(&self); }
trait BaseTrait: SuperTrait {}

impl SuperTrait for Unvalidated {
    fn foo(&self) {} //~ NOTE unvalidated
}
impl BaseTrait for Unvalidated {}

#[ferrocene::prevalidated] //~ NOTE marked
fn validated() { //~ NOTE is validated
    let x: &dyn BaseTrait = &Unvalidated; //~ ERROR unvalidated
    //~^ NOTE trait object
    //~^^ NOTE must assume
}
