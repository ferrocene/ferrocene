// Checks what happens when panicking inside the panic hook.

//@ run-crash
//@ exec-env:RUST_BACKTRACE=0
//@ check-run-results
//@ error-pattern: panicked while processing panic
//@ ignore-emscripten "RuntimeError" junk in output

// Ferrocene addition: Remove extra output line added by qemu
//@ normalize-stderr: "qemu: uncaught target signal 6 \(Aborted\) - core dumped\n" -> ""

use std::panic;

fn main() {
    panic::set_hook(Box::new(|_| panic::resume_unwind(Box::new(()))));
    panic!();
}
