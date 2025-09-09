#[link(name = "foo")]
extern "C" {
    #[link_ordinal(3, 4)]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE  expected a single argument
    //~| NOTE for more information, visit
    fn foo();
    #[link_ordinal(3, 4)]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE  expected a single argument
    //~| NOTE for more information, visit
    static mut imported_variable: i32;
}

fn main() {}

// ferrocene-annotations: fls_obik2w9gvhln
// Attribute link_ordinal
