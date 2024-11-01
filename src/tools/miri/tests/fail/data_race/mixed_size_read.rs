//@compile-flags: -Zmiri-preemption-rate=0.0 -Zmiri-disable-weak-memory-emulation
// Avoid accidental synchronization via address reuse inside `thread::spawn`.
//@compile-flags: -Zmiri-address-reuse-cross-thread-rate=0

use std::sync::atomic::{AtomicU8, AtomicU16, Ordering};
use std::thread;

fn convert(a: &AtomicU16) -> &[AtomicU8; 2] {
    unsafe { std::mem::transmute(a) }
}

// We can't allow mixed-size accesses; they are not possible in C++ and even
// Intel says you shouldn't do it.
fn main() {
    let a = AtomicU16::new(0);
    let a16 = &a;
    let a8 = convert(a16);

    thread::scope(|s| {
        s.spawn(|| {
            a16.load(Ordering::SeqCst);
        });
        s.spawn(|| {
            a8[0].load(Ordering::SeqCst);
            //~^ ERROR: Race condition detected between (1) 2-byte atomic load on thread `unnamed-1` and (2) 1-byte atomic load on thread `unnamed-2`
        });
    });
}
