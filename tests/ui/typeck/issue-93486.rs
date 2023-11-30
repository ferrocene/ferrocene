fn main() {
    while let 1 = 1 {
        vec![].last_mut().unwrap() = 3_u8;
        //~^ ERROR invalid left-hand side of assignment
    }
}

// ferrocene-annotations: fls_y4by2i8dl05o
// Assignment Expressions
//
// ferrocene-annotations: fls_3ut3biyra4r9
// Assignee Expressions
//
// ferrocene-annotations: fls_izdv9i4spokw
// Operator Expressions
