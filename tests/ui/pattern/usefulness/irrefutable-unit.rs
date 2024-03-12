//@ run-pass
//@ pretty-expanded FIXME #23616

pub fn main() {
    let ((),()) = ((),());
}

// ferrocene-annotations: fls_uh76pw6ykd57
// Refutability
