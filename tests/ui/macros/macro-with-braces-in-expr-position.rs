//@ run-pass
#![allow(unused_must_use)]
//@ ignore-emscripten no threads support

use std::thread;

macro_rules! expr { ($e: expr) => { $e } }

macro_rules! spawn {
    ($($code: tt)*) => {
        expr!(thread::spawn(move|| {$($code)*}).join())
    }
}

pub fn main() {
    spawn! {
        println!("stmt");
    };
    let _ = spawn! {
        println!("expr");
    };
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
//
// ferrocene-annotations: fls_4apk1exafxii
// Macro Matching
//
// ferrocene-annotations: fls_ym00b6ewf4n3
// Macro Transcription
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
//
// ferrocene-annotations: fls_n3ktmjqf87qb
// Rule Matching
//
// ferrocene-annotations: fls_qpx6lgapce57
// Token Matching
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
