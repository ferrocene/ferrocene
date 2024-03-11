//@ run-pass

#[repr(C)]
#[derive(Copy, Clone)]
struct Pair<T, U>(T, U);
#[repr(C)]
#[derive(Copy, Clone)]
struct Triple<T>(T, T, T);

#[repr(C)]
union U<A, B>
where
    A: Copy, B: Copy
{
    a: Pair<A, A>,
    b: B,
}

#[repr(C)]
union W<A, B>
where
    A: Copy, B: Copy
{
    a: A,
    b: B,
}

#[cfg(target_endian = "little")]
unsafe fn check() {
    let mut u = U::<u8, u16> { b: 0xDE_DE };
    u.a.0 = 0xBE;
    assert_eq!(u.b, 0xDE_BE);

    let mut u = U::<u16, u32> { b: 0xDEAD_DEAD };
    u.a.0 = 0xBEEF;
    assert_eq!(u.b, 0xDEAD_BEEF);

    let mut u = U::<u32, u64> { b: 0xDEADBEEF_DEADBEEF };
    u.a.0 = 0xBAADF00D;
    assert_eq!(u.b, 0xDEADBEEF_BAADF00D);

    let mut w = W::<Pair<Triple<u8>, u8>, u32> { b: 0xDEAD_DEAD };
    w.a.0 = Triple(0, 0, 0);
    assert_eq!(w.b, 0xDE00_0000);

    let mut w = W::<Pair<u8, Triple<u8>>, u32> { b: 0xDEAD_DEAD };
    w.a.1 = Triple(0, 0, 0);
    assert_eq!(w.b, 0x0000_00AD);
}

#[cfg(target_endian = "big")]
unsafe fn check() {
    let mut u = U::<u8, u16> { b: 0xDE_DE };
    u.a.0 = 0xBE;
    assert_eq!(u.b, 0xBE_DE);

    let mut u = U::<u16, u32> { b: 0xDEAD_DEAD };
    u.a.0 = 0xBEEF;
    assert_eq!(u.b, 0xBEEF_DEAD);

    let mut u = U::<u32, u64> { b: 0xDEADBEEF_DEADBEEF };
    u.a.0 = 0xBAADF00D;
    assert_eq!(u.b, 0xBAADF00D_DEADBEEF);

    let mut w = W::<Pair<Triple<u8>, u8>, u32> { b: 0xDEAD_DEAD };
    w.a.0 = Triple(0, 0, 0);
    assert_eq!(w.b, 0x0000_00AD);

    let mut w = W::<Pair<u8, Triple<u8>>, u32> { b: 0xDEAD_DEAD };
    w.a.1 = Triple(0, 0, 0);
    assert_eq!(w.b, 0xDE00_0000);
}

fn main() {
    unsafe {
        check();
    }
}

// ferrocene-annotations: fls_aibb2quva4mn
// Attribute repr
//
// ferrocene-annotations: fls_rjxpof29a3nl
// Struct Type Representation
//
// ferrocene-annotations: fls_cmq8ogs84ivh
// Union Type Representation
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_7nv8ualeaqe3
// Where Clauses
//
// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
//
// ferrocene-annotations: fls_u1afezy1ye99
// Conditional Compilation
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
//
// ferrocene-annotations: fls_18k3uajrgq5f
// Field Access Expressions
//
// ferrocene-annotations: fls_6ydylimiv553
// Place Expressions
//
// ferrocene-annotations: fls_xcwfotmq2e5d
// Field Resolution
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
