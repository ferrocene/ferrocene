// unit-test: Derefer
// EMIT_MIR derefer_inline_test.main.Derefer.diff
// EMIT_MIR_FOR_EACH_PANIC_STRATEGY

#![feature(box_syntax)]
#[inline]
fn f() -> Box<u32> {
    box 0
}
fn main() {
    box f();
}
