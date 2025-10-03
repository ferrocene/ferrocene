use super::*;

const GRCOV: &str = "743a0b97 ferrocene/tools/grcov (remotes/origin/HEAD)";
const FLIP_LINK: &str = "-b3118c82 ferrocene/tools/flip-link (v0.1.10-16-gb3118c8)";
const BACKTRACE: &str =
    "+b65ab935fb2e0d59dba8966ffca09c9cc5a5f57c library/backtrace (heads/master)";
const BOOK: &str = "+3e9dc46aa563ca0c53ec826c41b05f10c5915925 src/doc/book (3e9dc46aa)";

#[test]
fn parse_submodule_line_should_some() {
    assert_eq!(parse_submodule_line(GRCOV, "ferrocene/tools/grcov"), Some("743a0b97".into()));
    assert_eq!(
        parse_submodule_line(FLIP_LINK, "ferrocene/tools/flip-link"),
        Some("b3118c82".into())
    );
}

#[test]
fn parse_submodule_line_should_none() {
    assert_eq!(parse_submodule_line(FLIP_LINK, "ferrocene/tools/grcov"), None);
    assert_eq!(parse_submodule_line(GRCOV, "ferrocene/tools/flip-link"), None);
    assert_eq!(parse_submodule_line(BACKTRACE, "ferrocene/tools/grcov"), None);
    assert_eq!(parse_submodule_line(BOOK, "ferrocene/tools/grcov"), None);
}

#[test]
#[should_panic]
fn parse_submodule_line_should_panic_1() {
    parse_submodule_line(BACKTRACE, "library/backtrace");
}

#[test]
#[should_panic]
fn parse_submodule_line_should_panic_2() {
    parse_submodule_line(BOOK, "src/doc/book");
}
