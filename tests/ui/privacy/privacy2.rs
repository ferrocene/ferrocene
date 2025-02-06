//@ compile-flags: -Zdeduplicate-diagnostics=yes

#![feature(no_core)]
#![no_core] // makes debugging this test *a lot* easier (during resolve)

// Test to make sure that globs don't leak in regular `use` statements.

mod bar {
    pub use self::glob::*;

    pub mod glob {
        use foo;
    }
}

pub fn foo() {}
//~^ ERROR requires `sized` lang_item

fn test1() {
    //~^ ERROR requires `sized` lang_item
    use bar::foo;
    //~^ ERROR unresolved import `bar::foo` [E0432]
    //~| no `foo` in `bar`
}

fn test2() {
    //~^ ERROR requires `sized` lang_item
    use bar::glob::foo;
    //~^ ERROR `foo` is private
}

fn main() {}
<<<<<<< HEAD

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
=======
//~^ ERROR requires `sized` lang_item
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
