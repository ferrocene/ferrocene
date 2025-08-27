//@ build-fail
//@ normalize-stderr: "std::option::Option<\[u32; \d+\]>" -> "TYPE"
//@ normalize-stderr: "\[u32; \d+\]" -> "TYPE"

// FIXME(#61117): Respect debuginfo-level-tests, do not force debuginfo-level=0
//@ compile-flags: -Cdebuginfo=0

#[cfg(target_pointer_width = "32")]
type BIG = Option<[u32; (1<<29)-1]>;

#[cfg(target_pointer_width = "64")]
type BIG = Option<[u32; (1<<59)-1]>;

fn main() {
    let big: BIG = None;
    //~^ ERROR are too big for the target architecture
}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Type
//
// ferrocene-annotations: fls_fymvsy6ig99a
// Attribute cfg
