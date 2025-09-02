//@ compile-flags:-ldylib:+as-needed=foo -lstatic=bar -Zunstable-options

#[link(name = "foo")]
#[link(
    name = "bar",
    kind = "static",
)]
extern "C" {}
//~^ ERROR overriding linking modifiers from command line is not supported

fn main() {}

// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
