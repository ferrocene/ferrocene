use core::fmt;

// Covers `core::fmt::rt::Argument::<'_>::as_u16`
#[test]
fn test_rt_argument_as_u16_none() {
    fmt::ferrocene_test::test_rt_argument_as_u16_none();
}

// Covers `core::fmt::FormattingOptions::width`
#[test]
fn test_formatting_options_width() {
    let mut a = fmt::FormattingOptions::new();
    assert_eq!(a.width(None).get_width(), None);
}

// Covers `core::fmt::FormattingOptions::align`
#[test]
fn test_formatting_options_align_left() {
    let mut fo = fmt::FormattingOptions::new();
    let alignment = Some(fmt::Alignment::Left);
    assert_eq!(fo.align(alignment).get_align(), alignment);
}

// Covers `core::fmt::FormattingOptions::align`
#[test]
fn test_formatting_options_align_right() {
    let mut fo = fmt::FormattingOptions::new();
    let alignment = Some(fmt::Alignment::Right);
    assert_eq!(fo.align(alignment).get_align(), alignment);
}

// Covers `core::fmt::FormattingOptions::align`
#[test]
fn test_formatting_options_align_center() {
    let mut fo = fmt::FormattingOptions::new();
    let alignment = Some(fmt::Alignment::Center);
    assert_eq!(fo.align(alignment).get_align(), alignment);
}

// Covers `core::fmt::FormattingOptions::align`
#[test]
fn test_formatting_options_align_none() {
    let mut fo = fmt::FormattingOptions::new();
    let alignment = None;
    assert_eq!(fo.align(alignment).get_align(), alignment);
}
