// Regression test for #13428

fn foo() -> String {  //~ ERROR mismatched types
    format!("Hello {}",
            "world")
    // Put the trailing semicolon on its own line to test that the
    // note message gets the offending semicolon exactly
    ;
}

fn bar() -> String {  //~ ERROR mismatched types
    "foobar".to_string()
    ;
}

pub fn main() {}

// ferrocene-annotations: fls_1pg5ig740tg1
// Expression Statements
//
// ferrocene-annotations: fls_hndm19t57wby
// Block Expressions
