//@ force-host
//@ no-prefer-dynamic

#![crate_type = "proc-macro"]

#[macro_export]
macro_rules! foo { //~ ERROR cannot export macro_rules! macros from a `proc-macro` crate type
    ($e:expr) => ($e)
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
