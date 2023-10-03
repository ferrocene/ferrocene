// Test that return another type in place of ! raises a type mismatch.

fn fail() -> ! {
    return "wow"; //~ ERROR mismatched types
}

fn main() {
}

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
//
// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
