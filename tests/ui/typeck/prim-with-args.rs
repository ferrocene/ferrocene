//@ run-rustfix
fn main() {

let _x: isize<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: i8<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: i16<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: i32<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: i64<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: usize<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: u8<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: u16<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: u32<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: u64<isize>; //~ ERROR type arguments are not allowed on builtin type
let _x: char<isize>; //~ ERROR type arguments are not allowed on builtin type

let _x: isize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: i8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: i16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: i32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: i64<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: usize<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: u8<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: u16<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: u32<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: u64<'static>; //~ ERROR lifetime arguments are not allowed on builtin type
let _x: char<'static>; //~ ERROR lifetime arguments are not allowed on builtin type

}

// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_3qnpv2z7yjil
// Integer Types
//
// ferrocene-annotations: fls_wrvjizrqf3po
// Char Type
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
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
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
