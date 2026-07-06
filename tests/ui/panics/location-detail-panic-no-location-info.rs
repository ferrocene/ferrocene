//@ run-fail
//@ check-run-results
//@ compile-flags: -Zlocation-detail=none
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 (Aborted) - core dumped" -> ""

fn main() {
    panic!("no location info");
}
