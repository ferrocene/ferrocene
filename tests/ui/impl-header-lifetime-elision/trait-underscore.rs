// Test that `impl MyTrait<'_> for &i32` is equivalent to `impl<'a,
// 'b> MyTrait<'a> for &'b i32`.
//
//@ run-pass

#![allow(warnings)]

trait MyTrait<'a> { }

// This is equivalent to `MyTrait<'a> for &'b i32`, which is proven by
// the code below.
impl MyTrait<'_> for &i32 {
}

// When called, T will be `&'x i32` for some `'x`, so since we can
// prove that `&'x i32: for<'a> MyTrait<'a>, then we know that the
// lifetime parameter above is disconnected.
fn impls_my_trait<T: for<'a> MyTrait<'a>>() { }

fn impls_my_trait_val<T: for<'a> MyTrait<'a>>(_: T) {
    impls_my_trait::<T>();
}

fn random_where_clause()
where for<'a, 'b> &'a i32: MyTrait<'b> { }

fn main() {
    let x = 22;
    let f = &x;
    impls_my_trait_val(f);

    impls_my_trait::<&'static i32>();

    random_where_clause();
}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_85vx1qfa061i
// Traits
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
