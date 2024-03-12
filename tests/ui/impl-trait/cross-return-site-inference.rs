//@ edition:2021

fn foo(b: bool) -> impl std::fmt::Debug {
    if b {
        return vec![42]
    }
    [].into_iter().collect()
}

fn bar(b: bool) -> impl std::fmt::Debug {
    if b {
        return [].into_iter().collect()
    }
    vec![42]
}

fn bak(b: bool) -> impl std::fmt::Debug {
    if b {
        return std::iter::empty().collect()
    }
    vec![42]
}

fn baa(b: bool) -> impl std::fmt::Debug {
    if b {
        return [42].into_iter().collect()
    }
    vec![]
}

fn muh() -> Result<(), impl std::fmt::Debug> {
    Err("whoops")?;
    Ok(())
    //~^ ERROR type annotations needed
}

fn muh2() -> Result<(), impl std::fmt::Debug> {
    return Err(From::from("foo"));
    //~^ ERROR cannot call associated function on trait
    Ok(())
}

fn muh3() -> Result<(), impl std::fmt::Debug> {
    Err(From::from("foo"))
    //~^ ERROR cannot call associated function on trait
}

fn main() {}

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
//
// ferrocene-annotations: fls_1h0olpc7vbui
// Type Path Resolution
