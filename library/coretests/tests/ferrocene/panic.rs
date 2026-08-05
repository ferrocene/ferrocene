use std::fmt::Write;

#[test]
fn panic_location() {
    let loc = core::panic::Location::caller();

    let _ = loc.line();
    let _ = loc.column();
}

#[test]
fn panic_info() {
    let txt = "hello, world!";
    let args = format_args!("{txt}");
    let info = core::ferrocene_test::create_panic_info(&args);

    let s = info.to_string();
    eprintln!("{s}");
    let (fst, snd) = s.split_once(":\n").unwrap();
    assert!(fst.starts_with("panicked at "));
    assert_eq!(snd, txt);

    let msg = info.message();
    assert_eq!(format!("{msg:?}"), txt);
}

// Returns a given stream of responses to write calls
struct ErrorTriggerWriter<'a> {
    error_on: &'a str,
}

impl<'a> std::fmt::Write for ErrorTriggerWriter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if s == self.error_on {
            Err(std::fmt::Error)
        } else {
            Ok(())
        }
    }
}

// Covers <core::panic::panic_info::PanicInfo<'_> as core::fmt::Display>::fmt
#[test]
fn test_panic_info_fmt_results() {
    let args = format_args!("hello, world!");
    let info = core::ferrocene_test::create_panic_info(&args);

    let mut writer = ErrorTriggerWriter {
        error_on: "panicked at "
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "library/core/src/ferrocene_test.rs"
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ":\n"
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "hello, world!"
    };
    assert!(write!(writer, "{}", info).is_err());
}

// Covers <char as core::fmt::Debug>::fmt
#[test]
fn test_char_debug_fmt_errors() {
    let mut writer = ErrorTriggerWriter {
        error_on: "\'"
    };
    assert!(write!(writer, "{:?}", 'c').is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "c"
    };
    assert!(write!(writer, "{:?}", 'c').is_err());
}
