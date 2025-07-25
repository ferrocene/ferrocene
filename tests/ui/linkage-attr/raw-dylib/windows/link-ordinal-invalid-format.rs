#[link(name = "foo")]
extern "C" {
    #[link_ordinal("JustMonika")]
    //~^ ERROR malformed `link_ordinal` attribute input
    fn foo();
    #[link_ordinal("JustMonika")]
    //~^ ERROR malformed `link_ordinal` attribute input
    static mut imported_variable: i32;
}

fn main() {}

// ferrocene-annotations: fls_obik2w9gvhln
// Attribute link_ordinal
