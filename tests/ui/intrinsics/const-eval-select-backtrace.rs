#![feature(core_intrinsics)]
// See issue #100696.
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 \(Aborted\) - core dumped\n" -> ""

#[track_caller]
fn uhoh() {
    panic!("Aaah!")
}

const fn c() {}

fn main() {
    std::intrinsics::const_eval_select((), c, uhoh);
}
