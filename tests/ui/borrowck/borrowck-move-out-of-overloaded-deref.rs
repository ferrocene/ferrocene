use std::rc::Rc;

pub fn main() {
    let _x = *Rc::new("hi".to_string());
    //~^ ERROR cannot move out of an `Rc`
}

//
// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_5cm4gkt55hjh
// Dereference Expression/ References
