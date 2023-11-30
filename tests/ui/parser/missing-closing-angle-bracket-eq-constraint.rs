struct Foo<T1, T2> {
  _a : T1,
  _b : T2,
}

fn test1<T>(arg : T) {
  let v : Vec<(u32,_) = vec![];
    //~^ ERROR: expected one of
    //~| ERROR: type annotations needed
}

fn test2<T1, T2>(arg1 : T1, arg2 : T2) {
  let foo : Foo::<T1, T2 = Foo {_a : arg1, _b : arg2};
    //~^ ERROR: expected one of
}

fn test3<'a>(arg : &'a u32) {
  let v : Vec<'a = vec![];
    //~^ ERROR: expected one of
    //~| ERROR: type annotations needed for `Vec<T>`
}

fn main() {}

// ferrocene-annotations: fls_utuu8mdbuyxm
// Generic Arguments
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
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
