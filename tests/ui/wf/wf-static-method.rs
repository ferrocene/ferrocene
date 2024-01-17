// check that static methods can assume their trait-ref is well-formed.

trait Foo<'a, 'b, T>: Sized {
    fn make_me() -> Self { loop {} }
    fn static_evil(u: &'b u32) -> &'a u32;
}

struct Evil<'a, 'b: 'a>(Option<&'a &'b ()>);

impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
    fn make_me() -> Self { }
    fn static_evil(u: &'b u32) -> &'a u32 {
        u
    }
}

struct IndirectEvil<'a, 'b: 'a>(Option<&'a &'b ()>);

impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
    fn make_me() -> Self { IndirectEvil(None) }
    fn static_evil(u: &'b u32) -> &'a u32 {
        let me = Self::make_me();
        loop {} // (`me` could be used for the lifetime transmute).
    }
}

impl<'a, 'b> Evil<'a, 'b> {
    fn inherent_evil(u: &'b u32) -> &'a u32 {
        u
    }
}

// while static methods can *assume* this, we should still
// *check* that it holds at the use site.

fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
    <()>::static_evil(b)
    //~^ ERROR lifetime may not live long enough
}

fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
    <IndirectEvil>::static_evil(b)
    //~^ ERROR lifetime may not live long enough
}

fn inherent_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
    <Evil>::inherent_evil(b)
    //~^ ERROR lifetime may not live long enough
}


fn main() {}

// ferrocene-annotations: fls_85vx1qfa061i
// Traits
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_142vncdktbin
// Reference Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_e1pgdlv81vul
// Implementation Conformance
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
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
