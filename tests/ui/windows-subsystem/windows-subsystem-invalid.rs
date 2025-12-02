#![windows_subsystem = "wrong"]
//~^ ERROR malformed `windows_subsystem` attribute input [E0539]

fn main() {}

// ferrocene-annotations: fls_1l4mnlfk5rr2
// Attribute windows_subsystem
