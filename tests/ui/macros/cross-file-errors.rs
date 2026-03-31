#[macro_use]
mod underscore;

fn main() {
    underscore!();
    //~^ ERROR `_` can only be used on the left-hand side of an assignment
}

// ferrocene-annotations: fls_qxjy0f758x5s
// Attribute macro_use
