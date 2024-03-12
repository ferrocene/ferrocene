//@ compile-flags: --cfg something
//@ edition:2018

#![feature(async_closure)]
#![deny(unused_variables)]

extern "C" {
    fn ffi(
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        #[cfg_attr(something, cfg(nothing))] c: i32,
        #[cfg_attr(nothing, cfg(nothing))] ...
    );
}

type FnType = fn(
    #[cfg(nothing)] a: i32,
    #[cfg(something)] b: i32,
    #[cfg_attr(nothing, cfg(nothing))] c: i32,
    #[cfg_attr(something, cfg(nothing))] d: i32,
);

async fn foo_async(
    #[cfg(something)] a: i32,
    //~^ ERROR unused variable: `a`
    #[cfg(nothing)] b: i32,
) {}
fn foo(
    #[cfg(nothing)] a: i32,
    #[cfg(something)] b: i32,
    //~^ ERROR unused variable: `b`
    #[cfg_attr(nothing, cfg(nothing))] c: i32,
    //~^ ERROR unused variable: `c`
    #[cfg_attr(something, cfg(nothing))] d: i32,
) {}

struct RefStruct {}
impl RefStruct {
    async fn bar_async(
        &self,
        #[cfg(something)] a: i32,
        //~^ ERROR unused variable: `a`
        #[cfg(nothing)] b: i32,
    ) {}
    fn bar(
        &self,
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
    fn issue_64682_associated_fn(
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
}
trait RefTrait {
    fn bar(
        &self,
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
    fn issue_64682_associated_fn(
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
}
impl RefTrait for RefStruct {
    fn bar(
        &self,
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
    fn issue_64682_associated_fn(
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    ) {}
}

fn main() {
    let _: unsafe extern "C" fn(_, ...) = ffi;
    let _: fn(_, _) = foo;
    let _: FnType = |_, _| {};
    let a = async move |
        #[cfg(something)] a: i32,
        //~^ ERROR unused variable: `a`
        #[cfg(nothing)] b: i32,
    | {};
    let c = |
        #[cfg(nothing)] a: i32,
        #[cfg(something)] b: i32,
        //~^ ERROR unused variable: `b`
        #[cfg_attr(nothing, cfg(nothing))] c: i32,
        //~^ ERROR unused variable: `c`
        #[cfg_attr(something, cfg(nothing))] d: i32,
    | {};
    a(1);
    c(1, 2);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
// ferrocene-annotations: fls_l21tjqjkkaa0
// Associated Items
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
// ferrocene-annotations: fls_xztr1kebz8bo
// Function Pointer Types
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
// ferrocene-annotations: fls_dd9xh3wdjudo
// Attribute cfg_attr
//
// ferrocene-annotations: um_rustc_cfg
