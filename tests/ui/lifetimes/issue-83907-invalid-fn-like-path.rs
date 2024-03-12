//@ check-fail

static STATIC_VAR_FIVE: &One();
//~^ cannot find type
//~| free static item without body

fn main() {}

// ferrocene-annotations: fls_xdvdl2ssnhlo
// Statics
