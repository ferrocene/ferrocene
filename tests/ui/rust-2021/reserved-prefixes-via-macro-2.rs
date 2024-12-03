//@ edition:2018
//@ proc-macro: reserved-prefixes-macro-2018.rs
//@ proc-macro: reserved-prefixes-macro-2021.rs

extern crate reserved_prefixes_macro_2018 as m2018;
extern crate reserved_prefixes_macro_2021 as m2021;

fn main() {
    // Ok:
    m2018::number_of_tokens_in_a_prefixed_integer_literal!();
    m2018::number_of_tokens_in_a_prefixed_char_literal!();
    m2018::number_of_tokens_in_a_prefixed_string_literal!();

    // Error, even though *this* crate is 2018:
    m2021::number_of_tokens_in_a_prefixed_integer_literal!();
    //~^ ERROR prefix `hey` is unknown
    m2021::number_of_tokens_in_a_prefixed_char_literal!();
    //~^ ERROR prefix `hey` is unknown
    m2021::number_of_tokens_in_a_prefixed_string_literal!();
    //~^ ERROR prefix `hey` is unknown
}

// ferrocene-annotations: fls_gklst7joeo33
// External Crates
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
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
