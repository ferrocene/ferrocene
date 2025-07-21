#[link(name = "foo")]
extern "C" {
    #[link_ordinal()]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE  expected a single argument
    fn foo();
    #[link_ordinal()]
    //~^ ERROR malformed `link_ordinal` attribute input
    //~| NOTE  expected a single argument
    static mut imported_variable: i32;
}

fn main() {}

// ferrocene-annotations: fls_obik2w9gvhln
// Attribute link_ordinal
