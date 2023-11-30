type A = for<'a:> fn(); // OK
type A = for<'a:,> fn(); // OK
type A = for<'a> fn(); // OK
type A = for<> fn(); // OK
type A = for<'a: 'b + 'c> fn(); // OK (rejected later by ast_validation)
type A = for<'a: 'b,> fn(); // OK(rejected later by ast_validation)
type A = for<'a: 'b +> fn(); // OK (rejected later by ast_validation)
type A = for<'a, T> fn(); // OK (rejected later by ast_validation)
type A = for<,> fn(); //~ ERROR expected one of `#`, `>`, `const`, identifier, or lifetime

fn main() {}

// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
