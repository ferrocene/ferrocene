use core::{
    error::Error,
    num::{IntErrorKind, NonZero, ParseIntError, TryFromIntError},
};

#[test]
fn test_try_from_int_error() {
    let a: TryFromIntError = i8::try_from(u8::MAX).unwrap_err();

    // test fmt::Display::fmt
    let _b = format!("{}", a);

    // test error:Error::description
    #[allow(deprecated)]
    let _c = a.description();
}

#[test]
fn test_parse_int_error() {
    for (s, err_kind) in [
        ("", IntErrorKind::Empty),
        ("+", IntErrorKind::InvalidDigit),
        ("128", IntErrorKind::PosOverflow),
        ("-129", IntErrorKind::NegOverflow),
        ("0", IntErrorKind::Zero),
    ] {
        let a: ParseIntError = s.parse::<NonZero<i8>>().unwrap_err();

        // test fmt::Display::fmt
        let _b = format!("{}", a);

        // test error:Error::description
        #[allow(deprecated)]
        let _c = a.description();

        // test ParseIntError::kind
        // TODO: is not marked as executed in coverage report. is it because of const?
        let _d = a.kind();
        assert_eq!(_d, &err_kind)
    }
}
