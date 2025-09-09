fn main() {}

// fine
const Z: i32 = unsafe { *(&1 as *const i32) };

// bad, will thus error in miri
const Z2: i32 = unsafe { *(42 as *const i32) }; //~ ERROR dangling pointer
const Z3: i32 = unsafe { *(44 as *const i32) }; //~ ERROR dangling pointer

// ferrocene-annotations: fls_ppd1xwve3tr7
// Raw Pointer Type
