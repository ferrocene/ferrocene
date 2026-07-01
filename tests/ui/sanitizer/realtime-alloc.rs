//@ needs-sanitizer-support
//@ needs-sanitizer-realtime
//
//@ compile-flags: -Z sanitizer=realtime
//@ exec-env: RTSAN_OPTIONS=abort_on_error=0
//
//@ run-fail
//@ error-pattern: Intercepted call to real-time unsafe function `malloc` in real-time context!
//@ ignore-backends: gcc

// Ferrocene addition: Sanitizers are not a qualified compiler feature. Support on RHIVOS2 needs
// investigation.
//@ ignore-aarch64-rhivos2-linux-gnu

#![feature(sanitize)]

#[sanitize(realtime = "nonblocking")]
fn sanitizer_on() {
    let mut vec = vec![0, 1, 2];
    println!("alloc not detected");
}

fn main() {
    sanitizer_on();
}
