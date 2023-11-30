macro_rules! test_macro {
    ( $( $t:ty ),* $(),*) => {
        enum SomeEnum {
            $( $t, )* //~ ERROR expected identifier, found `String`
        };
    };
}

fn main() {
    test_macro!(String,);
}

// ferrocene-annotations: fls_8nzypdu9j3ge
// Metavariables
//
// ferrocene-annotations: fls_k01lsksqtq1r
// Repetition
