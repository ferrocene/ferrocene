//@ error-pattern: invalid windows subsystem `wrong`, only `windows` and `console` are allowed

#![windows_subsystem = "wrong"]

fn main() {}

// ferrocene-annotations: fls_1l4mnlfk5rr2
// Attribute windows_subsystem
