<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
//@ run-pass

#![feature(unsized_tuple_coercion)]

// Check that we do not change the offsets of ZST fields when unsizing

fn scalar_layout() {
    let sized: &(u8, [(); 13]) = &(123, [(); 13]);
    let unsize: &(u8, [()]) = sized;
    assert_eq!(sized.1.as_ptr(), unsize.1.as_ptr());
}

fn scalarpair_layout() {
    let sized: &(u8, u16, [(); 13]) = &(123, 456, [(); 13]);
    let unsize: &(u8, u16, [()]) = sized;
    assert_eq!(sized.2.as_ptr(), unsize.2.as_ptr());
}

pub fn main() {
    scalar_layout();
    scalarpair_layout();
}


// ferrocene-annotations: fls_4ckl3n2ko3i4
// Tuple Types
