#[allow(non_camel_case_types)]
trait foo { fn foo(&self); }

impl isize for usize { fn foo(&self) {} } //~ ERROR trait

fn main() {}

// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
