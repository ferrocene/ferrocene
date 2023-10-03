fn foo() -> bool {
    false
    //!self.allow_ty_infer()
    //~^ ERROR found doc comment
}

fn bar() -> bool {
    false
    /*! bar */ //~ ERROR found doc comment
}

fn baz() -> i32 {
    1 /** baz */ //~ ERROR found doc comment
}

fn quux() -> i32 {
    2 /*! quux */ //~ ERROR found doc comment
}

fn main() {}

// ferrocene-annotations: fls_q8l2jza7d9xa
// Comments
