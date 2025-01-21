//@ check-pass

fn raw_reborrow() {
    let x = &0;
    let y = &mut 0;

    let p = &raw const *x;
    let r = &raw const *y;
    let s = &raw mut *y;
}

unsafe fn raw_reborrow_of_raw() {
    let x = &0 as *const i32;
    let y = &mut 0 as *mut i32;

    let p = &raw const *x;
    let r = &raw const *y;
    let s = &raw mut *y;
}

fn main() {}

//
// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_vxguvrwolbee
// Raw Borrow Expression
