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

// Covers `core::fmt::Arguments::<'a>::estimated_capacity`
#[test]
fn test_arguments_estimated_capacity_n_128() {
    fn how_dare_you() -> &'static str {
        "How dare you!"
    }
    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. {}",
        how_dare_you()
    );
    assert_eq!(args.estimated_capacity(), 380);
}

// Covers `core::fmt::write`
#[test]
fn test_fmt_write_n_128() {
    fn how_dare_you() -> &'static str {
        "How dare you!"
    }
    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. {}",
        how_dare_you()
    );

    let mut buffer = String::new();
    fmt::write(&mut buffer, args).unwrap();

    assert!(buffer.starts_with("My message is that we'll be watching you."));
}

// Covers `core::fmt::Formatter::<'a>::pad_formatted_parts`
#[test]
fn test_formatter_pad_formatted_parts() {
    let mut options = fmt::FormattingOptions::new();

    // We have to trigger "if usize::from(width) <= len" but width has to be != 0
    options.width(Some(1));

    let mut buffer = String::new();
    let mut f = fmt::Formatter::new(&mut buffer, options);

    fmt::Display::fmt(&123.456, &mut f).unwrap();
    assert_eq!(buffer, "123.456");
}

// Covers `core::fmt::num::<impl u128>::_fmt_inner`
#[test]
fn test_u128_fmt_inner() {
    let mut buffer = String::new();
    let mut f = fmt::Formatter::new(&mut buffer, fmt::FormattingOptions::new());

    let n = 1_0000_0000_0000_0001_u128; // Needs to be greater or equal than 1_0000_0000_0000_0000
    fmt::Display::fmt(&n, &mut f).unwrap();
    assert_eq!(buffer, "10000000000000001");
}
