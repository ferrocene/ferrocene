//@ revisions: e2024 e2015
//@[e2024] edition: 2024
//@[e2015] edition: 2015

async gen fn foo() {}
//[e2015]~^ ERROR: `async fn` is not permitted in Rust 2015
//[e2015]~| ERROR: expected one of `extern`, `fn`, `safe`, or `unsafe`, found `gen`
//[e2024]~^^^ ERROR: gen blocks are experimental

fn main() {}
