//@ run-pass
//@ edition:2018

pub const A: usize = 0;

pub mod foo {
    pub const B: usize = 1;

    pub mod bar {
        pub const C: usize = 2;

        pub enum E {
            V1(usize),
            V2(String),
        }

        pub fn test() -> String {
            format!("{} {} {}", crate::A, crate::foo::B, C)
        }

        pub fn test_use() -> String {
            use crate::A;
            use crate::foo::B;

            format!("{} {} {}", A, B, C)
        }

        pub fn test_enum() -> String {
            use E::*;
            match E::V1(10) {
                V1(i) => { format!("V1: {}", i) }
                V2(s) => { format!("V2: {}", s) }
            }
        }
    }

    pub fn test() -> String {
        format!("{} {} {}", crate::A, B, bar::C)
    }

    pub fn test_use() -> String {
        use crate::A;
        use bar::C;

        format!("{} {} {}", A, B, C)
    }

    pub fn test_enum() -> String {
        use bar::E::*;
        match bar::E::V1(10) {
            V1(i) => { format!("V1: {}", i) }
            V2(s) => { format!("V2: {}", s) }
        }
    }
}

pub fn test() -> String {
    format!("{} {} {}", A, foo::B, foo::bar::C)
}

pub fn test_use() -> String {
    use foo::B;
    use foo::bar::C;

    format!("{} {} {}", A, B, C)
}

pub fn test_enum() -> String {
    use foo::bar::E::*;
    match foo::bar::E::V1(10) {
        V1(i) => { format!("V1: {}", i) }
        V2(s) => { format!("V2: {}", s) }
    }
}

fn main() {
    let output = [
        test(),
        foo::test(),
        foo::bar::test(),
        test_use(),
        foo::test_use(),
        foo::bar::test_use(),
        test_enum(),
        foo::test_enum(),
        foo::bar::test_enum(),
    ].join("\n");
    assert_eq!(output, "\
0 1 2
0 1 2
0 1 2
0 1 2
0 1 2
0 1 2
V1: 10
V1: 10
V1: 10");
}

// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_e9hwvqsib5d5
// Modules
//
// ferrocene-annotations: fls_szibmtfv117b
// Enum Types
//
// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
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
//
// ferrocene-annotations: fls_o9u2h5m17kpz
// Path Expression Resolution
