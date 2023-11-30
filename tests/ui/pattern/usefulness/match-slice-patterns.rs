fn check(list: &[Option<()>]) {
    match list {
    //~^ ERROR `&[_, Some(_), .., None, _]` not covered
        &[] => {},
        &[_] => {},
        &[_, _] => {},
        &[_, None, ..] => {},
        &[.., Some(_), _] => {},
    }
}

fn main() {}

// ferrocene-annotations: fls_e5td0fa92fay
// Match Expressions
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
