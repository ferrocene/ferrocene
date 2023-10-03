// compile-flags:-D bogus

// error-pattern:E0602
// error-pattern:requested on the command line with `-D bogus`

fn main() {}

// ferrocene-annotations: um_rustc_D
