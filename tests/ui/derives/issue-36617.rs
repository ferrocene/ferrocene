#![derive(Copy)]
//~^ ERROR `derive` attribute cannot be used at crate level

#![test]
//~^ ERROR `test` attribute cannot be used at crate level

#![test_case]
//~^ ERROR `test_case` attribute cannot be used at crate level

#![bench]
//~^ ERROR `bench` attribute cannot be used at crate level

#![global_allocator]
//~^ ERROR `global_allocator` attribute cannot be used at crate level

fn main() {}

// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
