struct A where (): Sized where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

fn b() where (): Sized where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

enum C where (): Sized where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

struct D where (): Sized, where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

fn e() where (): Sized, where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

enum F where (): Sized, where (): Sized {}
//~^ ERROR cannot define duplicate `where` clauses on an item

fn main() {}

// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
