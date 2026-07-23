//@ revisions: u w
//@[u] only-unix
//@[w] only-windows

#[path = "."]
mod m; //~ ERROR

fn main() {}

// ferrocene-annotations: fls_1zbaajz5prpn
// Attribute path
