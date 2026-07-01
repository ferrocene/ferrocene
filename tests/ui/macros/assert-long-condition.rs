//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0
// ignore-tidy-linelength

// Ferrocene addition: QEMU user space emulation outputs an extra message when an abort happens
//@ ignore-qemu

fn main() {
    assert!(1 + 2 + 3 + 4 + 5 + 6 + 7 + 8 + 9 + 10 + 11 + 12 + 13 + 14 + 15 + 16 + 17 + 18 + 19 + 20 + 21 + 22 + 23 + 24 + 25 == 0);
}
