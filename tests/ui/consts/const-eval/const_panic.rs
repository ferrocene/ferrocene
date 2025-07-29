//@ edition: 2015
#![allow(non_fmt_panics)]
#![crate_type = "lib"]

const MSG: &str = "hello";

const Z: () = std::panic!("cheese");
//~^ ERROR evaluation of constant value failed

const Z2: () = std::panic!();
//~^ ERROR evaluation of constant value failed

const Y: () = std::unreachable!();
//~^ ERROR evaluation of constant value failed

const X: () = std::unimplemented!();
//~^ ERROR evaluation of constant value failed

const W: () = std::panic!(MSG);
//~^ ERROR evaluation of constant value failed

const W2: () = std::panic!("{}", MSG);
//~^ ERROR evaluation of constant value failed

const Z_CORE: () = core::panic!("cheese");
//~^ ERROR evaluation of constant value failed

const Z2_CORE: () = core::panic!();
//~^ ERROR evaluation of constant value failed

const Y_CORE: () = core::unreachable!();
//~^ ERROR evaluation of constant value failed

const X_CORE: () = core::unimplemented!();
//~^ ERROR evaluation of constant value failed

const W_CORE: () = core::panic!(MSG);
//~^ ERROR evaluation of constant value failed

const W2_CORE: () = core::panic!("{}", MSG);
//~^ ERROR evaluation of constant value failed

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
