//@ edition:2015
use NonExistent; //~ ERROR unresolved import `NonExistent`
use non_existent::non_existent; //~ ERROR unresolved import `non_existent`

#[non_existent]
#[derive(NonExistent)]

struct S;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
