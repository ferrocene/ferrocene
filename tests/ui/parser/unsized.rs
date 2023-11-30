// Test syntax checks for `type` keyword.

struct S1 for type;
//~^ ERROR expected `where`, `{`, `(`, or `;` after struct name, found keyword `for`

pub fn main() {
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
