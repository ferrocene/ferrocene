//@ edition:2015
use foo::bar;
//~^ ERROR unresolved import `foo` [E0432]
//~| NOTE use of unresolved module or unlinked crate `foo`
//~| HELP you might be missing a crate named `foo`
//~| SUGGESTION extern crate foo;

use bar::Baz as x;
//~^ ERROR unresolved import `bar::Baz` [E0432]
//~| NOTE no `Baz` in `bar`
//~| HELP a similar name exists in the module
//~| SUGGESTION Bar

use food::baz;
//~^ ERROR unresolved import `food::baz`
//~| NOTE no `baz` in `food`
//~| HELP a similar name exists in the module
//~| SUGGESTION bag

use food::{beens as Foo};
//~^ ERROR unresolved import `food::beens` [E0432]
//~| NOTE no `beens` in `food`
//~| HELP a similar name exists in the module
//~| SUGGESTION beans

mod bar {
    pub struct Bar;
}

mod food {
    pub use self::zug::baz::{self as bag, Foobar as beans};

    mod zug {
        pub mod baz {
        //~^ NOTE module `food::zug::baz` exists but is inaccessible
        //~| NOTE not accessible
            pub struct Foobar;
        }
    }
}

mod m {
    enum MyEnum {
        MyVariant
    }

    use MyEnum::*;
    //~^ ERROR unresolved import `MyEnum` [E0432]
    //~| HELP a similar path exists
    //~| SUGGESTION self::MyEnum
}

mod items {
    enum Enum {
        Variant
    }

    use Enum::*;
    //~^ ERROR unresolved import `Enum` [E0432]
    //~| HELP a similar path exists
    //~| SUGGESTION self::Enum

    fn item() {}
}

fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
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
