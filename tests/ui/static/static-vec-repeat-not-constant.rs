fn foo() -> isize { 23 }

static a: [isize; 2] = [foo(); 2];
//~^ ERROR: E0015

fn main() {}

// ferrocene-annotations: fls_uj0kpjwyld60
// Array Types
//
// ferrocene-annotations: fls_xinykul167l
// Array Expressions
