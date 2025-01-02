// Check that `&raw mut` cannot be used to turn a `&T` into a `*mut T`.

fn raw_reborrow() {
    let x = &0;

    let q = &raw mut *x;                //~ ERROR cannot borrow
}

unsafe fn raw_reborrow_of_raw() {
    let x = &0 as *const i32;

    let q = &raw mut *x;                //~ ERROR cannot borrow
}

fn main() {}

//
// ferrocene-annotations: fls_v5x85lt5ulva
// References
//
// ferrocene-annotations: fls_vxguvrwolbee
// Raw Borrow Expression
