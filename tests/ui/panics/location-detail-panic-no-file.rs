//@ run-fail
//@ check-run-results
//@ compile-flags: -Zlocation-detail=line,column
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 \(Aborted\) - core dumped\n" -> ""

fn main() {
    panic!("file-redacted");
}
