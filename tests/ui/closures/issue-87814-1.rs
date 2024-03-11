//@ check-pass
fn main() {
    let mut schema_all = vec![];
    (0..42).for_each(|_x| match Err(()) as Result<(), _> {
        Ok(()) => schema_all.push(()),
        Err(_) => (),
    });
}

// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
