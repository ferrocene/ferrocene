//@ aux-build:crate_a1.rs
//@ aux-build:crate_a2.rs

// Issue 22750
// This tests the extra help message reported when a trait bound
// is not met but the struct implements a trait with the same path.

fn main() {
    let foo = {
        extern crate crate_a2 as a;
        a::Foo
    };

    let implements_no_traits = {
        extern crate crate_a2 as a;
        a::DoesNotImplementTrait
    };

    let other_variant_implements_mismatched_trait = {
        extern crate crate_a2 as a;
        a::ImplementsWrongTraitConditionally { _marker: std::marker::PhantomData::<isize> }
    };

    let other_variant_implements_correct_trait = {
        extern crate crate_a1 as a;
        a::ImplementsTraitForUsize { _marker: std::marker::PhantomData::<isize> }
    };

    {
        extern crate crate_a1 as a;
        a::try_foo(foo);
        //~^ ERROR E0277
        //~| trait impl with same name found
        //~| perhaps two different versions of crate `crate_a2`

        // We don't want to see the "version mismatch" help message here
        // because `implements_no_traits` has no impl for `Foo`
        a::try_foo(implements_no_traits);
        //~^ ERROR E0277

        // We don't want to see the "version mismatch" help message here
        // because `other_variant_implements_mismatched_trait`
        // does not have an impl for its `<isize>` variant,
        // only for its `<usize>` variant.
        a::try_foo(other_variant_implements_mismatched_trait);
        //~^ ERROR E0277

        // We don't want to see the "version mismatch" help message here
        // because `ImplementsTraitForUsize` only has
        // impls for the correct trait where the path is not misleading.
        a::try_foo(other_variant_implements_correct_trait);
        //~^ ERROR E0277
        //~| the trait `main::a::Bar` is implemented for `ImplementsTraitForUsize<usize>`
    }
}

// ferrocene-annotations: fls_jeoas4n6su4
// Trait and Lifetime Bounds
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_maw4u1o8q37u
// Crates
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
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
