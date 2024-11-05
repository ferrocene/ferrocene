fn main() {
    || {
        if false {
            return "test";
        }
        let a = true;
        a //~ ERROR mismatched types
    };

    || -> bool {
        if false {
            return "hello" //~ ERROR mismatched types
        };
        let b = true;
        b
    };
}

<<<<<<< HEAD
// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_8l74abhlxzdl
// Return Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
=======
// issue: rust-lang/rust#130858 rust-lang/rust#125655
static FOO: fn() -> bool = || -> bool { 1 };
//~^ ERROR mismatched types
>>>>>>> pull-upstream-temp--do-not-use-for-real-code
