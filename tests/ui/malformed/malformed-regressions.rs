#[doc] //~ ERROR attribute must be of the form
//~^ WARN this was previously accepted
#[ignore()] //~ ERROR attribute must be of the form
//~^ WARN this was previously accepted
#[inline = ""] //~ ERROR attribute must be of the form
//~^ WARN this was previously accepted
#[link] //~ ERROR attribute must be of the form
//~^ WARN this was previously accepted
#[link = ""] //~ ERROR attribute must be of the form
//~^ WARN this was previously accepted

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
