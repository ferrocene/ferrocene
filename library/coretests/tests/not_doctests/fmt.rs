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

/// This horrific type exists because `&mut dyn fmt::Write` does not hit the
/// specialisation for unsized types in `fmt::Write::write_fmt`.
#[repr(C)]
struct UnsizedBuffer<const N: usize> {
    len: usize,
    buffer: [u8],
}

impl<const N: usize> UnsizedBuffer<N> {
    const LEN_SIZE: usize = size_of::<usize>();
    const CAPACITY: usize = N.checked_sub(Self::LEN_SIZE).expect("buffer too small to hold index");

    /// Construct a new [UnsizedBuffer] within the `buffer`.
    fn new(buffer: &mut [u8; N]) -> &mut Self {
        let ptr = buffer as *mut [u8] as *mut Self;
        // SAFETY: initialise the length field, so that writing to the buffer does not overwrite the length
        unsafe { (*ptr).len = Self::LEN_SIZE };
        // SAFETY: we hold a unique reference to the buffer
        unsafe { &mut *ptr }
    }

    fn as_str(&self) -> &str {
        str::from_utf8(&self.buffer[Self::LEN_SIZE..self.len]).unwrap()
    }
}

impl<const N: usize> fmt::Write for UnsizedBuffer<N> {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let bytes = s.as_bytes();
        let new_len = self.len + bytes.len();

        // check that buffer has enough space
        if new_len > Self::CAPACITY {
            return fmt::Result::Err(fmt::Error);
        }

        self.buffer[self.len..new_len].copy_from_slice(bytes);
        self.len = new_len;

        Ok(())
    }
}

// Covers `core::fmt::Write::write_fmt`
#[test]
fn test_write_fmt_unsized() {
    let mut array = [0u8; 256];
    let buffer = UnsizedBuffer::new(&mut array);

    fn how_dare_you() -> &'static str {
        "How dare you!"
    }
    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. {}",
        how_dare_you()
    );
    fmt::Write::write_fmt(buffer, args).unwrap();
    assert_eq!(
        buffer.as_str(),
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. How dare you!"
    )
}

// Covers `core::fmt::Write::write_fmt`
#[test]
fn test_write_fmt_unsized_statically_known() {
    let mut array = [0_u8; 256];
    let buffer = UnsizedBuffer::new(&mut array);

    fmt::Write::write_fmt(
        buffer,
        format_args!(
            "My message is that we'll be watching you. This is all wrong. I shouldn't be up here."
        ),
    )
    .unwrap();
    assert_eq!(
        buffer.as_str(),
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here."
    )
}

// Covers `core::fmt::Write::write_fmt`
#[test]
fn test_write_fmt_sized_statically_known() {
    let mut buffer = String::new();
    fmt::Write::write_fmt(&mut buffer, format_args!("Hello, world!")).unwrap();
    assert_eq!(buffer.as_str(), "Hello, world!")
}
