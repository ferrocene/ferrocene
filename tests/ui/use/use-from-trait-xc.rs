//@ aux-build:use-from-trait-xc.rs

extern crate use_from_trait_xc;

use use_from_trait_xc::Trait::foo;
//~^ ERROR `foo` is not directly importable

use use_from_trait_xc::Trait::Assoc;
//~^ ERROR `Assoc` is not directly importable

use use_from_trait_xc::Trait::CONST;
//~^ ERROR `CONST` is not directly importable

use use_from_trait_xc::Foo::new; //~ ERROR struct `Foo` is private
//~^ ERROR unresolved import `use_from_trait_xc::Foo`

use use_from_trait_xc::Foo::C; //~ ERROR struct `Foo` is private
//~^ ERROR unresolved import `use_from_trait_xc::Foo`

use use_from_trait_xc::Bar::new as bnew;
//~^ ERROR unresolved import `use_from_trait_xc::Bar`

use use_from_trait_xc::Baz::new as baznew;
//~^ ERROR unresolved import `use_from_trait_xc::Baz::new`

fn main() {}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
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
