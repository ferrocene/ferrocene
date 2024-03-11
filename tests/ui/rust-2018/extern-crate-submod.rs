//@ aux-build:edition-lint-paths.rs
//@ run-rustfix

// Oddball: extern crate appears in a submodule, making it harder for
// us to rewrite paths. We don't (and we leave the `extern crate` in
// place).

#![deny(absolute_paths_not_starting_with_crate)]

mod m {
    // Because this extern crate does not appear at the root, we
    // ignore it altogether.
    pub extern crate edition_lint_paths;
}

// And we don't being smart about paths like this, even though you
// *could* rewrite it to `use edition_lint_paths::foo`
use m::edition_lint_paths::foo;
//~^ ERROR absolute paths must start
//~| WARNING this is accepted in the current edition

fn main() {
    foo();
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
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
