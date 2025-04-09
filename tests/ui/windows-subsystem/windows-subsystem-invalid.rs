#![windows_subsystem = "wrong"]

fn main() {}

//~? ERROR invalid windows subsystem `wrong`

// ferrocene-annotations: fls_1l4mnlfk5rr2
// Attribute windows_subsystem
