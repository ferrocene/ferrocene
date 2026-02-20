//@ add-minicore
//@ check-fail

#![deny(ferrocene::unvalidated)]
#![allow(internal_features)]
#![feature(intrinsics)]
#![crate_type = "lib"]

const fn unvalidated() {}

#[rustc_intrinsic]
#[ferrocene::prevalidated]
const fn prefetch_read_data<T, const LOCALITY: i32>(_data: *const T) {
    unvalidated(); //~ ERROR unvalidated
}

const fn prefetch_read<T>(_ptr: *const T, _locality: Locality) {}

enum Locality {
    L3,
    L2,
    L1,
}

#[ferrocene::prevalidated]
fn uses_unvalidated_intrinsic() {
    let x = 1;
    prefetch_read(&x, Locality::L3); //~ ERROR unvalidated
}
