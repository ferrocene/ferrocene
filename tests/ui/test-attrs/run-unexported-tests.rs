//@ run-fail
//@ compile-flags:--test
//@ check-stdout

mod m {
    pub fn exported() {}

    #[test]
    fn unexported() {
        panic!("ran an unexported test");
    }
}

// ferrocene-annotations: um_rustc_test
