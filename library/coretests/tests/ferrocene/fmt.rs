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
    let mut array = [0; 256];
    let buffer = UnsizedBuffer::new(&mut array);

    fn how_dare_you() -> &'static str {
        "How dare you!"
    }
    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. {}",
        how_dare_you()
    );
    assert!(args.as_statically_known_str().is_none());

    fmt::Write::write_fmt(buffer, args).unwrap();
    assert_eq!(
        buffer.as_str(),
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. How dare you!"
    )
}

mod module_to_avoid_optimisations {
    use super::*;

    // Covers `core::fmt::Write::write_fmt`
    #[test]
    fn test_write_fmt_unsized_statically_known() {
        let mut array = [0; 256];
        let buffer = UnsizedBuffer::new(&mut array);

        let args = format_args!("Hello, world!");
        assert!(args.as_statically_known_str().is_some());

        fmt::Write::write_fmt(buffer, args).unwrap();

        assert_eq!(buffer.as_str(), "Hello, world!")
    }
}

// Covers `core::fmt::Write::write_fmt`
#[test]
fn test_write_fmt_sized_statically_known() {
    let mut buffer = String::new();

    let args = format_args!("Hello, world!");
    assert!(args.as_statically_known_str().is_some());

    fmt::Write::write_fmt(&mut buffer, args).unwrap();
    assert_eq!(buffer.as_str(), "Hello, world!")
}

//  Test `core::fmt::Formatter::<'a>::debug_struct_field._finish`
macro_rules! test_debug_struct_field_finish {
    ($($fn:ident => $T:ident { $($field:ident),* } == $str:literal,)*) => {
        $(
        #[test]
        #[allow(dead_code, unused)]
        fn $fn() {
            #[derive(Debug, Default)]
            struct $T {
                $($field: (),)*
            }
            assert_eq!(
                format!("{:?}", <$T>::default()),
                $str,
            );
        }
        )*
    }
}
test_debug_struct_field_finish! {
    test_debug_struct_field1_finish => StructField1 { a } == "StructField1 { a: () }",
    test_debug_struct_field2_finish => StructField2 { a, b } == "StructField2 { a: (), b: () }",
    test_debug_struct_field3_finish => StructField3 { a, b, c } == "StructField3 { a: (), b: (), c: () }",
    test_debug_struct_field4_finish => StructField4 { a, b, c, d } == "StructField4 { a: (), b: (), c: (), d: () }",
    test_debug_struct_field5_finish => StructField5 { a, b, c, d, e } == "StructField5 { a: (), b: (), c: (), d: (), e: () }",
    test_debug_struct_fields_finish => StructFields { a, b, c, d, e, f } == "StructFields { a: (), b: (), c: (), d: (), e: (), f: () }",
}
//  Test `core::fmt::Formatter::<'a>::debug_tuple_field._finis(h`
macro_rules! test_debug_tuple_field_finish {
    ($($fn:ident => $T:ident ( $($t:ty),* ) == $str:literal,)*) => {
        $(
        #[test]
        #[allow(dead_code, unused)]
        fn $fn() {
            #[derive(Debug, Default)]
            struct $T ( $($t,)* );
            assert_eq!(
                format!("{:?}", <$T>::default()),
                $str,
            );
        }
        )*
    }
}
test_debug_tuple_field_finish! {
    test_debug_tuple_field1_finish => TupleField1(()) == "TupleField1(())",
    test_debug_tuple_field2_finish => TupleField2((), ()) == "TupleField2((), ())",
    test_debug_tuple_field3_finish => TupleField3((), (), ()) == "TupleField3((), (), ())",
    test_debug_tuple_field4_finish => TupleField4((), (), (), ()) == "TupleField4((), (), (), ())",
    test_debug_tuple_field5_finish => TupleField5((), (), (), (), ()) == "TupleField5((), (), (), (), ())",
    test_debug_tuple_fields_finish => TupleFields((), (), (), (), (), ()) == "TupleFields((), (), (), (), (), ())",
}

// Covers `<&mut T as core::fmt::Pointer>::fmt`
#[test]
fn mut_ref_fmt_pointer() {
    assert!(format!("{:p}", &mut [1, 2, 3]).starts_with("0x"));
}

// Covers:
// - `<(dyn core::any::Any + core::marker::Send + 'static) as core::fmt::Debug>::fmt`
// - `<(dyn core::any::Any + core::marker::Send + core::marker::Sync + 'static) as core::fmt::Debug>::fmt`
#[test]
fn dyn_any_send_static_fmt_debug() {
    use core::any::Any;
    use core::marker::{Send, Sync};

    let a = "Hello, world!";

    let b: &(dyn Any + Send + 'static) = &a;
    assert_eq!(format!("{:?}", b), "Any { .. }");

    let c: &(dyn Any + Send + Sync + 'static) = &a;
    assert_eq!(format!("{:?}", c), "Any { .. }");
}
