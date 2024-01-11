extern "C" {
    static mut a: i32;
}

fn main() {
    a += 3; //~ ERROR: requires unsafe
    a = 4; //~ ERROR: requires unsafe
    let _b = a; //~ ERROR: requires unsafe
}

// ferrocene-annotations: fls_jep7p27kaqlp
// Unsafety
