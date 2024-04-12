// Verifies that MIR building for a call expression respects
// privacy when checking if a call return type is uninhabited.
//@ run-rustfix
#![allow(unreachable_code, unused_variables)]

pub mod widget {
    enum Unimplemented {}
    pub struct Widget(Unimplemented);

    impl Widget {
        pub fn new() -> Widget {
            todo!();
        }
    }

    pub fn f() {
        let x: &mut u32;
        Widget::new();
        // Ok. Widget type returned from new is known to be uninhabited
        // and the following code is considered unreachable.
        *x = 1;
    }
}

fn main() {
    let y: &mut u32;
    widget::Widget::new();
    // Error. Widget type is not known to be uninhabited here,
    // so the following code is considered reachable.
    *y = 2; //~ ERROR E0381
}

// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_jdknpu3kf865
// Visibility
//
// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
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
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
