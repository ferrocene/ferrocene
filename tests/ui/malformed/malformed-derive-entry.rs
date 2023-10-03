#[derive(Copy(Bad))]
//~^ ERROR traits in `#[derive(...)]` don't accept arguments
//~| ERROR the trait bound
struct Test1;

#[derive(Copy="bad")]
//~^ ERROR traits in `#[derive(...)]` don't accept values
//~| ERROR the trait bound
struct Test2;

#[derive] //~ ERROR malformed `derive` attribute input
struct Test4;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
