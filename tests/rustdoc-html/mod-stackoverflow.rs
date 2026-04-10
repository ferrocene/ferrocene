//@ aux-build:mod-stackoverflow.rs
//@ edition: 2015
//@ ignore-cross-compile

extern crate mod_stackoverflow;
pub use mod_stackoverflow::tree;
pub use mod_stackoverflow::tree2;
