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

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_8l74abhlxzdl
// Return Expressions
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
