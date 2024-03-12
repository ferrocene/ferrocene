//@ check-pass

fn main() {
    let mut schema_all: (Vec<String>, Vec<String>) = (vec![], vec![]);

    let _c = || match schema_all.0.try_reserve(1) as Result<(), _> {
        Ok(()) => (),
        Err(_) => (),
    };
}

// ferrocene-annotations: fls_jmjn8jkbzujm
// Capturing
//
// ferrocene-annotations: fls_8gpcpvc99pxj
// Call Conformance
