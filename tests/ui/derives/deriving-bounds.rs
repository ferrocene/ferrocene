#[derive(Send)]
//~^ ERROR cannot find derive macro `Send` in this scope
//~| ERROR cannot find derive macro `Send` in this scope
struct Test;

#[derive(Sync)]
//~^ ERROR cannot find derive macro `Sync` in this scope
//~| ERROR cannot find derive macro `Sync` in this scope
struct Test1;

pub fn main() {}

// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
