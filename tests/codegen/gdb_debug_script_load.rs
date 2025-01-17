//
//@ ignore-windows
//@ ignore-apple
//@ ignore-wasm
//@ ignore-emscripten
//@ ignore-thumbv7em-ferrocenecoretest-eabi target disables GDB script loading
//@ ignore-thumbv7em-ferrocenecoretest-eabihf target disables GDB script loading

//@ compile-flags: -g -C no-prepopulate-passes

#![feature(start)]

// CHECK-LABEL: @main
// CHECK: load volatile i8, {{.+}} @__rustc_debug_gdb_scripts_section__

#[start]
fn start(_: isize, _: *const *const u8) -> isize {
    return 0;
}
