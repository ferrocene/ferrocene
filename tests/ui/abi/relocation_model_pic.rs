// run-pass
// compile-flags: -C relocation-model=pic
// ignore-emscripten no pic
// ignore-wasm
// ignore-aarch64-unknown-ferrocenecoretest no pic

#![feature(cfg_relocation_model)]

#[cfg(relocation_model = "pic")]
fn main() {}
