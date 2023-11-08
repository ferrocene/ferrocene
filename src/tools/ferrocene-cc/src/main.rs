//! Script to invoke the system C compiler with the appropriate PATH so it runs
//! our `rust-lld` wrapper (called `ld`), not the system linker (usually
//! `/usr/bin/ld`).

fn main() {
    println!("Hello, world!");
}
