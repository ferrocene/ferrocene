#[link(name = "foo")]
extern "C" {
    #[link_ordinal()]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE expected an argument here
    //~| NOTE for more information, visit
    fn foo();
    #[link_ordinal()]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE expected an argument here
    //~| NOTE for more information, visit
    static mut imported_variable: i32;
}

fn main() {}

// ferrocene-annotations: fls_obik2w9gvhln
// Attribute link_ordinal
