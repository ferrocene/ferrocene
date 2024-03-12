//@ edition:2018

#![allow(non_camel_case_types)]
#![allow(unused_imports)]

mod T {
    pub struct U;
}
mod x {
    pub struct y;
}

fn type_param<T>() {
    use T as _; //~ ERROR imports cannot refer to type parameters
    use T::U; //~ ERROR imports cannot refer to type parameters
    use T::*; //~ ERROR imports cannot refer to type parameters
}

fn self_import<T>() {
    use T; //~ ERROR imports cannot refer to type parameters
}

fn let_binding() {
    let x = 10;

    use x as _; //~ ERROR imports cannot refer to local variables
    use x::y; // OK
    use x::*; // OK
}

fn param_binding(x: u8) {
    use x; //~ ERROR imports cannot refer to local variables
}

fn match_binding() {
    match 0 {
        x => {
            use x; //~ ERROR imports cannot refer to local variables
        }
    }
}

fn nested<T>() {
    let x = 10;

    use {T as _, x}; //~ ERROR imports cannot refer to type parameters
                     //~| ERROR imports cannot refer to local variables
}

fn main() {}

// ferrocene-annotations: fls_9gprp17h6t1q
// Use Imports
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
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
