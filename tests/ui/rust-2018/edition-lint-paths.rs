//@ aux-build:edition-lint-paths.rs
//@ run-rustfix

#![deny(absolute_paths_not_starting_with_crate)]
#![allow(unused)]

extern crate edition_lint_paths;

pub mod foo {
    use edition_lint_paths;
    use bar::Bar;
    //~^ ERROR absolute
    //~| WARN this is accepted in the current edition

    use super::bar::Bar2;
    use crate::bar::Bar3;

    use bar;
    //~^ ERROR absolute
    //~| WARN this is accepted in the current edition

    use crate::bar as something_else;

    use {main, Bar as SomethingElse};
    //~^ ERROR absolute
    //~| WARN this is accepted in the current edition
    //~| ERROR absolute
    //~| WARN this is accepted in the current edition
    //~| ERROR absolute
    //~| WARN this is accepted in the current edition

    use crate::{main as another_main, Bar as SomethingElse2};

    pub fn test() {}

    pub trait SomeTrait {}
}

use bar::Bar;
//~^ ERROR absolute
//~| WARN this is accepted in the current edition

pub mod bar {
    use edition_lint_paths as foo;
    pub struct Bar;
    pub type Bar2 = Bar;
    pub type Bar3 = Bar;
}

mod baz {
    use *;
    //~^ ERROR absolute
    //~| WARN this is accepted in the current edition
}

impl ::foo::SomeTrait for u32 {}
//~^ ERROR absolute
//~| WARN this is accepted in the current edition

fn main() {
    let x = ::bar::Bar;
    //~^ ERROR absolute
    //~| WARN this is accepted in the current edition

    let x = bar::Bar;
    let x = crate::bar::Bar;
    let x = self::bar::Bar;
    foo::test();

    {
        use edition_lint_paths as bar;
        edition_lint_paths::foo();
        bar::foo();
        ::edition_lint_paths::foo();
    }
}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_151r19d7xbgz
// Entities
//
// ferrocene-annotations: fls_izl8iuhoz9e0
// Scopes
//
// ferrocene-annotations: fls_6ozthochxz1i
// Binding Scopes
//
// ferrocene-annotations: fls_ftphlagzd2te
// Generic Parameter Scope
//
// ferrocene-annotations: fls_m0z7omni9hp0
// Item Scope
//
// ferrocene-annotations: fls_769b4p8v3cwu
// Label Scope
//
// ferrocene-annotations: fls_kgbi26212eof
// Self Scope
//
// ferrocene-annotations: fls_octf6sf7yso
// Textual Macro Scope
//
// ferrocene-annotations: fls_lnpyb285qdiy
// Scope Hierarchy
//
// ferrocene-annotations: fls_dq403wq5yrs
// Namespaces
//
// ferrocene-annotations: fls_ld0ize96cm6m
// Preludes
//
// ferrocene-annotations: fls_ydmnb7qnmzzq
// Shadowing
//
// ferrocene-annotations: fls_40xoego2thsp
// Resolution
//
// ferrocene-annotations: fls_i6qzga6dyaee
// Path Resolution
//
// ferrocene-annotations: fls_bbso3c45kr9z
// Simple Path Resolution
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
