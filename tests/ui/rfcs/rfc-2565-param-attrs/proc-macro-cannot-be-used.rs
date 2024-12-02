//@ proc-macro: ident-mac.rs

#![feature(c_variadic)]
#![allow(anonymous_parameters)]

extern crate ident_mac;
use ident_mac::id;

struct W(u8);

extern "C" { fn ffi(#[id] arg1: i32, #[id] ...); }
//~^ ERROR expected non-macro attribute, found attribute macro
//~| ERROR expected non-macro attribute, found attribute macro

unsafe extern "C" fn cvar(arg1: i32, #[id] mut args: ...) {}
//~^ ERROR expected non-macro attribute, found attribute macro

type Alias = extern "C" fn(#[id] u8, #[id] ...);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro

fn free(#[id] arg1: u8) {
    //~^ ERROR expected non-macro attribute, found attribute macro
    let lam = |#[id] W(x), #[id] y: usize| ();
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
}

impl W {
    fn inherent1(#[id] self, #[id] arg1: u8) {}
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn inherent2(#[id] &self, #[id] arg1: u8) {}
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn inherent3<'a>(#[id] &'a mut self, #[id] arg1: u8) {}
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn inherent4<'a>(#[id] self: Box<Self>, #[id] arg1: u8) {}
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8) {}
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
}

trait A {
    fn trait1(#[id] self, #[id] arg1: u8);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn trait2(#[id] &self, #[id] arg1: u8);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn trait3<'a>(#[id] &'a mut self, #[id] arg1: u8);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn trait4<'a>(#[id] self: Box<Self>, #[id] arg1: u8, #[id] Vec<u8>);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
    fn issue_64682_associated_fn<'a>(#[id] arg1: u8, #[id] arg2: u8);
    //~^ ERROR expected non-macro attribute, found attribute macro
    //~| ERROR expected non-macro attribute, found attribute macro
}

fn main() {}


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
// ferrocene-annotations: fls_wn1i6hzg2ff7
// Procedural Macros
