//@ run-fail
//@ error-pattern:explicit panic
//@ ignore-emscripten no processes

fn main() {
    let _x = if false {
        0
    } else if true {
        panic!()
    } else {
        10
    };
}

// ferrocene-annotations: fls_mkut7gut49gi
// If Expressions
