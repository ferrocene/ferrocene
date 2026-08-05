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

// Covers `core::cell::panic_already_borrowed::do_panic`
#[test]
#[should_panic(expected = "RefCell already borrowed")]
fn test_const_panic_do_panic() {
    use std::cell::RefCell;

    let c = RefCell::new(5);
    let _m = c.borrow();

    let _b = c.borrow_mut(); // this causes a panic
}

// Covers `core::char::methods::encode_utf8_raw::do_panic`
#[test]
#[should_panic(expected = "encode_utf8")]
fn test_runtime_const_panic() {
    let code = std::hint::black_box('ß' as u32);
    let mut buf = [0; 1];

    let _ = std::char::encode_utf8_raw(code, &mut buf);
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
