const A: [&str; 1] = { "hello" };
//~^ ERROR mismatched types

const B: &[u32] = &{ 1 };
//~^ ERROR mismatched types

const C: &&[u32; 1] = &&{ 1 };
//~^ ERROR mismatched types

fn main() {}

// ferrocene-annotations: fls_exe4zodlwfez
// Type Unification
