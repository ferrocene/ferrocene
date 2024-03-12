//@ run-pass

fn f<T>(t: T) -> T {
    let t1 = t;
    t1
}

pub fn main() {
    let t = f::<Box<_>>(Box::new(100));
    assert_eq!(t, Box::new(100));
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_wttihxen35as
// Constant Promotion
