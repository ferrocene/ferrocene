//! A stub linker driver that does not support `-fuse-ld`

use std::{env, process};

fn main() {
    for arg in env::args() {
        if arg.starts_with("-fuse-ld") {
            // matches the error message that GCC emits and rustc uses to implement the
            // -fuse-ld fallback
            // see https://github.com/ferrocene/ferrocene/blob/4045dc3753f/compiler/rustc_codegen_ssa/src/back/link.rs#L930 @ 4045dc3753f
            eprintln!("error: unrecognized command line option '{arg}'");
            process::exit(1)
        }
    }

    unreachable!("invoked without -fuse-ld flag!");
}
