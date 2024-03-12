//@ run-pass

pub fn main() {
    fn as_buf<T, F>(s: String, f: F) -> T where F: FnOnce(String) -> T { f(s) }
    as_buf("foo".to_string(), |foo: String| -> () { println!("{}", foo) });
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
