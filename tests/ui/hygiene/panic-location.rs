//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0
//@ normalize-stderr: ".rs:\d+:\d+" -> ".rs:LL:CC"
//@ normalize-stderr: "/rustc(?:-dev)?/[a-z0-9.]+/" -> ""

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 \(Aborted\) - core dumped\n" -> ""
//
// Regression test for issue #70963
// The reported panic location should not be `<::core::macros::panic macros>`.
fn main() {
    std::collections::VecDeque::<String>::with_capacity(!0);
}
