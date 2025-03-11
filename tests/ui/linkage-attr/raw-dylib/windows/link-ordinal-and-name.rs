#[link(name="foo")]
extern "C" {
    #[link_name="foo"]
    #[link_ordinal(42)]
    //~^ ERROR cannot use `#[link_name]` with `#[link_ordinal]`
    fn foo();
    #[link_name="foo"]
    #[link_ordinal(5)]
    //~^ ERROR cannot use `#[link_name]` with `#[link_ordinal]`
    static mut imported_variable: i32;
}

fn main() {}

// ferrocene-annotations: fls_p44fky7fifc
// Attribute link_name
//
// ferrocene-annotations: fls_obik2w9gvhln
// Attribute link_ordinal
