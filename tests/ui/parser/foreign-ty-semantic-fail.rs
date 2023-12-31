#![feature(extern_types)]

fn main() {}

extern "C" {
    type A: Ord;
    //~^ ERROR bounds on `type`s in `extern` blocks have no effect
    type B<'a> where 'a: 'static;
    //~^ ERROR `type`s inside `extern` blocks cannot have generic parameters
    //~| ERROR `type`s inside `extern` blocks cannot have `where` clauses
    type C<T: Ord> where T: 'static;
    //~^ ERROR `type`s inside `extern` blocks cannot have generic parameters
    //~| ERROR `type`s inside `extern` blocks cannot have `where` clauses
    type D = u8;
    //~^ ERROR incorrect `type` inside `extern` block

    type E: where;
    //~^ ERROR `type`s inside `extern` blocks cannot have `where` clauses
}

// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
//
// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
//
// ferrocene-annotations: fls_kgvleup5mdhq
// Type Aliases
