// rust-lang/rust#53675: At one point the compiler errored when a test
// named `panic` used the `assert!` macro in expression position.

//@ check-pass
//@ compile-flags: --test

mod in_expression_position {
    #[test]
    fn panic() {
        assert!(true)
    }
}

mod in_statement_position {
    #[test]
    fn panic() {
        assert!(true);
    }
}

mod what_if_we_use_panic_directly_in_expr {
    #[test]
    #[should_panic]
    fn panic() {
        panic!("in expr")
    }
}


mod what_if_we_use_panic_directly_in_stmt {
    #[test]
    #[should_panic]
    fn panic() {
        panic!("in stmt");
    }
}

// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_aes2d94g12b9
// Attribute should_panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
//
// ferrocene-annotations: um_rustc_test
