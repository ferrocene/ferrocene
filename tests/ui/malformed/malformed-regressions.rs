#[doc] //~ ERROR valid forms for the attribute are
//~^ WARN this was previously accepted
#[ignore()] //~ ERROR valid forms for the attribute are
//~^ WARN this was previously accepted
#[inline = ""] //~ ERROR valid forms for the attribute are
//~^ WARN this was previously accepted
#[link] //~ ERROR malformed
#[link = ""] //~ ERROR malformed

fn main() {}

// ferrocene-annotations: fls_63v1fqedzwfd
// Attribute doc
//
// ferrocene-annotations: fls_x849a4u7h82j
// Attribute ignore
//
// ferrocene-annotations: fls_ypio6boj3pwf
// Attribute inline
//
// ferrocene-annotations: fls_o0f9ae22ug1x
// Attribute link
