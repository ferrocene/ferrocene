//@ aux-build:no_method_suggested_traits.rs
extern crate no_method_suggested_traits;

struct Foo;
enum Bar { X }

mod foo {
    pub trait Bar {
        fn method(&self) {}

        fn method2(&self) {}
    }

    impl Bar for u32 {}

    impl Bar for char {}
}

fn main() {
    // test the values themselves, and autoderef.


    1u32.method();
    //~^ ERROR no method named
    //~|items from traits can only be used if the trait is in scope
    std::rc::Rc::new(&mut Box::new(&1u32)).method();
    //~^items from traits can only be used if the trait is in scope
    //~| ERROR no method named `method` found for struct

    'a'.method();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&'a')).method();
    //~^ ERROR no method named

    1i32.method();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&1i32)).method();
    //~^ ERROR no method named

    Foo.method();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&Foo)).method();
    //~^ ERROR no method named

    1u64.method2();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&1u64)).method2();
    //~^ ERROR no method named

    no_method_suggested_traits::Foo.method2();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method2();
    //~^ ERROR no method named
    no_method_suggested_traits::Bar::X.method2();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method2();
    //~^ ERROR no method named

    Foo.method3();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&Foo)).method3();
    //~^ ERROR no method named
    Bar::X.method3();
    //~^ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&Bar::X)).method3();
    //~^ ERROR no method named

    // should have no help:
    1_usize.method3(); //~ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&1_usize)).method3(); //~ ERROR no method named
    no_method_suggested_traits::Foo.method3();  //~ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Foo)).method3();
    //~^ ERROR no method named
    no_method_suggested_traits::Bar::X.method3();  //~ ERROR no method named
    std::rc::Rc::new(&mut Box::new(&no_method_suggested_traits::Bar::X)).method3();
    //~^ ERROR no method named
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_9i5msiuuyihf
// Paths
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
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
