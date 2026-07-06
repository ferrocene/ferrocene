//@ run-fail
//@ check-run-results
//@ compile-flags: -Copt-level=0 -Zlocation-detail=line,column
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 (Aborted) - core dumped" -> ""

fn main() {
    let opt: Option<u32> = None;
    opt.unwrap();
}
