#[derive(Copy(Bad))]
//~^ ERROR: traits in `#[derive(...)]` don't accept arguments
struct Test1;
//~^ ERROR: the trait bound

#[derive(Copy="bad")]
//~^ ERROR: traits in `#[derive(...)]` don't accept values
struct Test2;
//~^ ERROR: the trait bound

#[derive] //~ ERROR: malformed `derive` attribute input
struct Test4;

fn main() {}

// ferrocene-annotations: fls_r6gj1p4gajnq
// Attribute derive
