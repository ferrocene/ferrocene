//@ aux-build:mod-stackoverflow.rs
//@ ignore-cross-compile
//@ edition: 2015

extern crate mod_stackoverflow;
pub use mod_stackoverflow::tree;
pub use mod_stackoverflow::tree2;
