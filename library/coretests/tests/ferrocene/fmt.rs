use core::fmt::{self, Write};

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

// Cover `core::fmt::write`'s try branches
#[test]
fn test_write_try_branches() {
    fn how_dare_you() -> &'static str {
        "How dare you!"
    }

    let mut writer = ErrorTriggerWriter {
        error_on: "My message is that we'll be watching you. This is all wrong. \
    I shouldn't be up here. I should be back in school on the other side of the ocean. \
    Yet you all come to us young people for hope. ",
        allow_times: 0,
    };

    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. \
    I shouldn't be up here. I should be back in school on the other side of the ocean. \
    Yet you all come to us young people for hope. {}",
        how_dare_you()
    );

    assert!(fmt::write(&mut writer, args).is_err());
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

// Cover `core::fmt::Formatter::<'a>::write_formatted_parts`'s try branches
#[test]
fn test_formatter_write_formatted_parts_try_branches() {
    // We avoid digging into `numfmt::Part` by using `{:e}`.

    // Cover `self.buf.write_str(formatted.sign)?;`
    {
        let mut writer = ErrorTriggerWriter {
            error_on: "-",
            allow_times: 0,
        };
        let mut f = fmt::Formatter::new(&mut writer, fmt::FormattingOptions::new());
        assert!(fmt::Display::fmt(&-1.0_f64, &mut f).is_err());
    }

    // Cover `self.buf.write_str(ZEROES)?;`
    {
        let mut writer = ErrorTriggerWriter {
            error_on: &"0".repeat(64),
            allow_times: 0,
        };
        let mut options = fmt::FormattingOptions::new();
        options.precision(Some(65));
        let mut f = fmt::Formatter::new(&mut writer, options);
        assert!(fmt::LowerExp::fmt(&1_u64, &mut f).is_err());
    }

    // Cover `self.buf.write_str(&ZEROES[..nzeroes])?;`
    {
        let mut writer = ErrorTriggerWriter {
            error_on: "0",
            allow_times: 0,
        };
        let mut options = fmt::FormattingOptions::new();
        options.precision(Some(1));
        let mut f = fmt::Formatter::new(&mut writer, options);
        assert!(fmt::LowerExp::fmt(&1_u64, &mut f).is_err());
    }

    // Cover `write_bytes(self.buf, &s[..len])?;`
    {
        let mut writer = ErrorTriggerWriter {
            error_on: "0",
            allow_times: 0,
        };
        let mut f = fmt::Formatter::new(&mut writer, fmt::FormattingOptions::new());
        assert!(fmt::LowerExp::fmt(&1.0_f64, &mut f).is_err());
    }
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

// Covers `core::fmt::Formatter::<'a>::sign`
#[test]
fn test_formatter_sign_none() {
    let mut buffer = String::new();
    let f = fmt::Formatter::new(&mut buffer, fmt::FormattingOptions::new());

    assert_eq!(f.sign(), None);
}

// Covers `core::fmt::Formatter::<'a>::sign`
#[test]
fn test_formatter_sign_some() {
    let mut options = fmt::FormattingOptions::new();
    options.sign(Some(fmt::Sign::Plus));

    let mut buffer = String::new();
    let f = fmt::Formatter::new(&mut buffer, options);

    assert_eq!(f.sign(), Some(fmt::Sign::Plus));
}

// Covers `core::fmt::Formatter::<'a>::with_options`
#[test]
fn test_formatter_with_options() {
    // these are the first options, they gonna be overwritten
    let mut options1 = fmt::FormattingOptions::new();
    options1.precision(Some(4));

    // create a formatter with the first options
    let mut buffer = String::new();
    let mut f1 = fmt::Formatter::new(&mut buffer, options1);

    // these are the second options, they gonna be used
    let mut options2 = fmt::FormattingOptions::new();
    options2.sign(Some(fmt::Sign::Plus));
    let f2 = f1.with_options(options2);

    // assert that the first options are overwritten
    assert_ne!(f2.precision(), Some(4));
    // assert that the second options are used
    assert_eq!(f2.sign(), Some(fmt::Sign::Plus));
}

// Covers `core::fmt::FormattingOptions::create_formatter`
#[test]
fn test_formatting_options_create_formatter() {
    let mut options = fmt::FormattingOptions::new();
    options.sign(Some(fmt::Sign::Plus));

    let mut buffer = String::new();
    let f = options.create_formatter(&mut buffer);

    assert_eq!(f.sign(), Some(fmt::Sign::Plus));
}

// Covers `core::fmt::FormattingOptions::precision`
#[test]
fn test_formatting_options_precision_none() {
    let mut options = fmt::FormattingOptions::new();

    options.precision(None);
    assert_eq!(options.get_precision(), None);
}

// Covers `core::fmt::FormattingOptions::precision`
#[test]
fn test_formatting_options_precision_some() {
    let mut options = fmt::FormattingOptions::new();

    options.precision(Some(4));
    assert_eq!(options.get_precision(), Some(4));
}

// Covers `core::fmt::builders::DebugList::<'a, 'b>::entry_with`
#[test]
fn test_builder_debug_list_entry_with() {
    struct Foo(Vec<i32>);

    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_list().entry_with(|f| write!(f, "new entry: {:?}", self.0)).finish()
        }
    }

    assert_eq!(format!("{:?}", Foo(vec![1, 2, 3])), "[new entry: [1, 2, 3]]");
}

// Covers `core::fmt::builders::DebugSet::<'a, 'b>::entry_with`
#[test]
fn test_builder_debug_set_entry_with() {
    struct Foo(Vec<i32>);

    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_set().entry_with(|f| write!(f, "new entry: {:?}", self.0)).finish()
        }
    }

    assert_eq!(format!("{:?}", Foo(vec![1, 2, 3])), "{new entry: [1, 2, 3]}");
}

// Covers `core::fmt::builders::from_fn`
#[test]
fn test_builder_from_fn() {
    let value = 5;
    let wrapped = fmt::from_fn(|f| {
        for _ in 0..value {
            write!(f, "from_fn ")?
        }
        write!(f, "from_fn")
    });
    assert_eq!(format!("{wrapped}"), "from_fn from_fn from_fn from_fn from_fn from_fn");
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

// Covers:
// - `core::fmt::Write::write_fmt`
// - `<&mut W as core::fmt::Write>::write_fmt`
// - `<&mut W as core::fmt::Write>::write_char`
#[test]
fn test_write_fmt_unsized() {
    let mut array = [0; 256];
    let mut buffer = UnsizedBuffer::new(&mut array);

    fn how_dare_you() -> &'static str {
        "How dare you"
    }
    let args = format_args!(
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. {}",
        how_dare_you()
    );
    assert!(args.as_statically_known_str().is_none());

    fmt::Write::write_fmt(&mut buffer, args).unwrap();
    fmt::Write::write_char(&mut buffer, '!').unwrap();
    assert_eq!(
        buffer.as_str(),
        "My message is that we'll be watching you. This is all wrong. I shouldn't be up here. I should be back in school on the other side of the ocean. Yet you all come to us young people for hope. How dare you!"
    );
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

// Covers:
// - `<core::fmt::Formatter<'_> as core::fmt::Write>::write_fmt`
// - `<core::fmt::Formatter<'_> as core::fmt::Write>::write_str`
#[test]
fn test_formatter_write() {
    struct Foo;

    impl fmt::Display for Foo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt::Write::write_fmt(f, format_args!("Hello"))?;
            for x in [",", " "] {
                fmt::Write::write_fmt(f, format_args!("{x}"))?;
            }
            fmt::Write::write_str(f, "world!")
        }
    }

    assert_eq!(Foo.to_string(), "Hello, world!");
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

// Covers `<core::any::TypeId as core::fmt::Debug>::fmt`
#[test]
fn any_type_id_fmt_debug() {
    assert!(format!("{:?}", core::any::Any::type_id(&0_u32)).starts_with("TypeId(0x"));
}

// Cover `<core::array::TryFromSliceError as core::fmt::Display>::fmt`
#[test]
fn array_try_from_slice_error_fmt_display() {
    assert_eq!(
        format!("{}", TryInto::<[u32; 4]>::try_into([0].as_slice()).unwrap_err()),
        "could not convert slice to array"
    );
}

// Cover `<core::ascii::EscapeDefault as core::fmt::Debug>::fmt`
#[test]
fn ascii_escape_default_fmt_debug() {
    assert_eq!(format!("{:?}", core::ascii::escape_default(b'\t')), "EscapeDefault { .. }");
}

// Cover `<core::cell::SyncUnsafeCell<T> as core::fmt::Debug>::fmt`
#[test]
fn cell_sync_unsafe_cell_fmt_debug() {
    assert_eq!(format!("{:?}", core::cell::SyncUnsafeCell::new(0)), "SyncUnsafeCell { .. }");
}

// Cover `<core::cell::UnsafeCell<T> as core::fmt::Debug>::fmt`
#[test]
fn cell_unsafe_cell_fmt_debug() {
    assert_eq!(format!("{:?}", core::cell::UnsafeCell::new(0)), "UnsafeCell { .. }");
}

// Cover `<core::escape::EscapeIterInner<N, core::escape::AlwaysEscaped> as core::fmt::Debug>::fmt`
#[test]
fn escape_iter_inner_always_escaped_fmt_debug() {
    assert_eq!(format!("{:?}", '?'.escape_unicode()), "EscapeUnicode(EscapeIterInner('\\u{3f}'))");
}

// Cover `<core::escape::EscapeIterInner<N, core::escape::MaybeEscaped> as core::fmt::Debug>::fmt`
#[test]
fn escape_iter_inner_maybe_escaped_fmt_debug() {
    assert_eq!(format!("{:?}", '!'.escape_debug()), "EscapeDebug(EscapeIterInner('!'))");
}

// Cover `<core::ffi::c_void as core::fmt::Debug>::fmt`
#[test]
fn ffi_c_void_fmt_debug() {
    assert_eq!(format!("{:?}", core::ffi::c_void::__variant1), "c_void");
}

// Cover `<core::fmt::Error as core::fmt::Display>::fmt`
#[test]
fn fmt_error_fmt_display() {
    assert_eq!(format!("{:?}", core::fmt::Error), "Error");
}

// Covers `<core::marker::PhantomData<T> as core::fmt::Debug>::fmt`.
#[test]
fn phantom_data_debug_fmt() {
    assert_eq!(format!("{:?}", core::marker::PhantomData::<()>), "PhantomData<()>");
}

// Covers `<core::fmt::Error as core::fmt::Display>::fmt`.
#[test]
#[should_panic = "an error occurred when formatting an argument"]
fn fmt_error_display_fmt() {
    panic!("{}", core::fmt::Error)
}

// Covers `<core::fmt::builders::FromFn<F> as core::fmt::Debug>::fmt`.
#[test]
fn from_fn_debug_fmt() {
    let msg = "Hello, world!";

    let val = core::fmt::from_fn(|f| write!(f, "{msg}"));
    assert_eq!(format!("{val:?}"), msg);
}

// Covers `core::fmt::Formatter::<'a>::fill`
// Copied from `core::fmt::Formatter::<'a>::fill` doc test
#[test]
fn test_formatter_fill() {
    struct Foo;

    impl fmt::Display for Foo {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            let c = formatter.fill();
            if let Some(width) = formatter.width() {
                for _ in 0..width {
                    write!(formatter, "{c}")?;
                }
                Ok(())
            } else {
                write!(formatter, "{c}")
            }
        }
    }

    // We set alignment to the right with ">".
    assert_eq!(format!("{Foo:G>3}"), "GGG");
    assert_eq!(format!("{Foo:t>6}"), "tttttt");
}

// Covers `core::fmt::Formatter::<'a>::align`
// Copied from `core::fmt::Formatter::<'a>::align` doc test
#[test]
fn test_formatter_align() {
    use std::fmt::{self, Alignment};

    struct Foo;

    impl fmt::Display for Foo {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            let s = if let Some(s) = formatter.align() {
                match s {
                    Alignment::Left => "left",
                    Alignment::Right => "right",
                    Alignment::Center => "center",
                }
            } else {
                "into the void"
            };
            write!(formatter, "{s}")
        }
    }

    assert_eq!(format!("{Foo:<}"), "left");
    assert_eq!(format!("{Foo:>}"), "right");
    assert_eq!(format!("{Foo:^}"), "center");
    assert_eq!(format!("{Foo}"), "into the void");
}


// Returns a given stream of responses to write calls
struct ErrorTriggerWriter<'a> {
    error_on: &'a str,
    allow_times: usize,
}

impl<'a> std::fmt::Write for ErrorTriggerWriter<'a> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        if s == self.error_on && self.allow_times == 0 {
            println!("Blocked str {s:#?}");
            Err(std::fmt::Error)
        } else {
            if s == self.error_on {
                self.allow_times -= 1;
            }
            Ok(())
        }
    }
    fn write_char(&mut self, c: char) -> std::fmt::Result {
        if c.to_string() == self.error_on && self.allow_times == 0 {
            println!("Blocked char {c:#?}");
            Err(std::fmt::Error)
        } else {
            if c.to_string() == self.error_on {
                self.allow_times -= 1;
            }
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
        error_on: "panicked at ",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "library/core/src/ferrocene_test.rs",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ":\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", info).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "hello, world!",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", info).is_err());
}

// Covers <char as core::fmt::Debug>::fmt
#[test]
fn test_char_debug_fmt_errors() {
    let mut writer = ErrorTriggerWriter {
        error_on: "\'",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", 'c').is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "c",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", 'c').is_err());
}

// Covers `<core::fmt::builders::PadAdapter<'_, '_> as core::fmt::Write>::write_str`
#[test]
fn test_pad_adapter_fmt_write_str_errors() {
    struct DemoStruct {
        #[allow(dead_code)]
        demo: &'static str,
        #[allow(dead_code)]
        hidden: &'static str,
    }
    impl fmt::Debug for DemoStruct {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_struct("DemoStruct")
               .field("demo", &self.demo)
               .finish_non_exhaustive() // Show that some other field(s) exist.
        }
    }
    let demo = DemoStruct { demo: "demo1\ndemo2", hidden: "Whoops" };
    let mut writer = ErrorTriggerWriter {
        error_on: "    ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "demo1",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "demo2",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "..\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());
}


// Covers `<core::fmt::builders::PadAdapter<'_, '_> as core::fmt::Write>::write_char`
#[test]
fn test_pad_adapter_fmt_write_char_errors() {
    #[allow(dead_code)]
    #[derive(Debug)]
    struct DemoStruct(char, char);
    let demo = DemoStruct('x', 'w');
    let mut writer = ErrorTriggerWriter {
        error_on: "    ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "x",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "w",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "    ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", demo).is_err());
}

// Covers `<core::bstr::ByteStr as core::fmt::Debug>::fmt`'s try branches
#[test]
fn test_bytestr_fmt_errors() {
    use std::bstr::ByteString;
    // The first characters are: 狐仙
    let mut buf = b"\xe6\x8b\x90\xe4\xbb\x99\n1\n2\03\x014".to_vec();
    buf.push(223); // xDFFF - Invalid
    buf.push(255);
    let specimen = ByteString(buf);

    let mut writer = ErrorTriggerWriter {
        error_on: "仙",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\"",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "1",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\0",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\x01",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\"",
        allow_times: 1,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\xff",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers `<core::ops::range::Range<Idx> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_range_fmt_errors() {
    let specimen = 1..10;

    let mut writer = ErrorTriggerWriter {
        error_on: "1",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "..",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "10",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers `<core::ops::range::RangeTo<Idx> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_range_to_fmt_errors() {
    let specimen = ..10;

    let mut writer = ErrorTriggerWriter {
        error_on: "..",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "10",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers `<core::ops::range::RangeInclusive<Idx> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_range_inclusive_fmt_errors() {
    let specimen = 0..=10;

    let mut writer = ErrorTriggerWriter {
        error_on: "0",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());


    let mut writer = ErrorTriggerWriter {
        error_on: "..=",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "10",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut specimen = 10..=10;
    specimen.next();
    let mut writer = ErrorTriggerWriter {
        error_on: " (exhausted)",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers `<core::ops::range::RangeToInclusive<Idx> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_range_to_inclusive_fmt_errors() {
    let specimen = ..=10;

    let mut writer = ErrorTriggerWriter {
        error_on: "..=",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "10",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers `<core::ops::range::RangeFrom<Idx> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_range_from_fmt_errors() {
    let specimen = 10..;

    let mut writer = ErrorTriggerWriter {
        error_on: "10",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "..",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

}

// Covers errors in:
// * <core::num::imp::bignum::Big32x40 as core::fmt::Debug>::fmt
// * <core::num::imp::bignum::tests::Big8x3 as core::fmt::Debug>::fmt
#[test]
fn test_bignum_fmt_errors() {
    use core::num::imp::bignum::Big32x40;
    let specimen = Big32x40::from_u64(0xffffffffffffffff);

    let mut writer = ErrorTriggerWriter {
        error_on: "0x",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "ffffffff",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());


    let mut writer = ErrorTriggerWriter {
        error_on: "_",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "ffffffff",
        allow_times: 1,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Cover `core::fmt::Formatter::<'a>::pad`'s errors
#[test]
fn test_formatter_pad_errors() {
    let specimen = "hello";

    let mut writer = ErrorTriggerWriter {
        error_on: " ",
        allow_times: 4,
    };
    assert!(write!(writer, "{:<10}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "hello",
        allow_times: 0,
    };
    assert!(write!(writer, "{:>10}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: " ",
        allow_times: 3,
    };
    assert!(write!(writer, "{:>10}", specimen).is_err());
}

// Cover `core::fmt::Formatter::<'a>::pad_formatted_parts`'s errors
#[test]
fn test_formatter_pad_formatted_parts_errors() {
    let mut options = fmt::FormattingOptions::new();

    // We have to trigger "if usize::from(width) <= len" but width has to be != 0
    options.width(Some(100));
    options.sign_aware_zero_pad(true);
    options.sign(Some(core::fmt::Sign::Plus));
    options.align(Some(core::fmt::Alignment::Right));

    let mut writer = ErrorTriggerWriter {
        error_on: "+",
        allow_times: 0,
    };
    let mut f = fmt::Formatter::new(&mut writer, options);
    assert!(fmt::Display::fmt(&123.456, &mut f).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "0",
        allow_times: 10,
    };
    let mut f = fmt::Formatter::new(&mut writer, options);
    assert!(fmt::Display::fmt(&123.456, &mut f).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ".",
        allow_times: 0,
    };
    let mut f = fmt::Formatter::new(&mut writer, options);
    assert!(fmt::Display::fmt(&123.456, &mut f).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "456",
        allow_times: 0,
    };
    let mut f = fmt::Formatter::new(&mut writer, options);
    assert!(fmt::Display::fmt(&123.456, &mut f).is_err());
}

// Cover `<core::slice::ascii::EscapeAscii<'a> as core::fmt::Display>::fmt`'s errors
#[test]
fn test_escape_ascii_fmt_errors() {
    let specimen = b"0\t\r\nabc'\"\\\x9d";
    let mut escaped = specimen.escape_ascii();
    escaped.next();
    escaped.next();
    escaped.next_back();

    let mut writer = ErrorTriggerWriter {
        error_on: "t",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());


    let mut writer = ErrorTriggerWriter {
        error_on: "\\'",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\\"",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "abc",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "x",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "9",
        allow_times: 0,
    };
    assert!(write!(writer, "{}", escaped).is_err());
}

// Cover `<core::time::Duration as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_duration_fmt_errors() {
    use std::time::Duration;
    let specimen = Duration::new(5, 0);
    let mut writer = ErrorTriggerWriter {
        error_on: "0",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0<50?}", specimen).is_err()); // L1542

    let mut writer = ErrorTriggerWriter {
        error_on: "+",
        allow_times: 0,
    };
    assert!(write!(writer, "{:+?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: " ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:^15?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "5",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0<50?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "s",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0<50?}", specimen).is_err());


    let mut writer = ErrorTriggerWriter {
        error_on: ".",
        allow_times: 0,
    };
    assert!(write!(writer, "{:.20?}", specimen).is_err());

    let max_round_up_specimen = Duration::new(u64::MAX, 999_999_999);
    let mut writer = ErrorTriggerWriter {
        error_on: "18446744073709551616",
        allow_times: 0,
    };
    assert!(write!(writer, "{:.1?}", max_round_up_specimen).is_err());
}

// Cover `<str as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_str_fmt_errors() {
    let specimen = "hello\t";
    let mut writer = ErrorTriggerWriter {
        error_on: "\"",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\\t",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());
}

// Cover `<core::str::iter::Chars<'_> as core::fmt::Debug>::fmt`'s errors
#[test]
fn test_chars_fmt_errors() {
    let specimen = "hello".chars();

    let mut writer = ErrorTriggerWriter {
        error_on: "Chars(",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "h",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ")",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());
}

// Cover `core::fmt::Formatter::<'a>::pad_integral`'s errors
#[test]
fn test_pad_integral_errors() {
    // Taken from the `pad_integral` doctest
    use std::fmt;
    struct Foo { nb: i32 }

    impl fmt::Display for Foo {
        fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            // We need to remove "-" from the number output.
            let tmp = self.nb.abs().to_string();

            formatter.pad_integral(self.nb >= 0, "Foo ", &tmp)
        }
    }

    let specimen = Foo { nb: -5 };

    let mut writer = ErrorTriggerWriter {
        error_on: "0",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0>#8}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "-",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0>#8}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "Foo ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0>#8}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "5",
        allow_times: 0,
    };
    assert!(write!(writer, "{:0>#8}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "5",
        allow_times: 0,
    };
    assert!(write!(writer, "{:016}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "0",
        allow_times: 0,
    };
    assert!(write!(writer, "{:016}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "Foo ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#016}", specimen).is_err());
}

// Cover `core::fmt::FormattingOptions::get_width`'s branches
#[test]
fn test_formatting_options_get_width() {
    let mut options = core::fmt::FormattingOptions::default();
    assert_eq!(options.get_width(), None);
    options.width(Some(5));
    assert_eq!(options.get_width(), Some(5));
}

// Cover `core::fmt::builders::DebugSet::<'a, 'b>::finish_non_exhaustive`'s try branches
#[test]
fn test_debug_set_finish_non_exhaustive_try_branches() {
    use std::fmt;

    struct Foo(Vec<i32>);

    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Print at most two elements, abbreviate the rest
            let mut f = fmt.debug_set();
            let f = f.entries(self.0.iter().take(2));
            if self.0.len() > 2 {
                f.finish_non_exhaustive()
            } else {
                f.finish()
            }
        }
    }

    let mut writer = ErrorTriggerWriter {
        error_on: "..\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", Foo(vec![1, 2, 3, 4])).is_err());
}

// Cover `core::fmt::builders::DebugList::<'a, 'b>::finish_non_exhaustive`'s try branches
#[test]
fn test_debug_list_finish_non_exhaustive_try_branches() {
    use std::fmt;

    struct Foo(Vec<i32>);

    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Print at most two elements, abbreviate the rest
            let mut f = fmt.debug_list();
            let f = f.entries(self.0.iter().take(2));
            if self.0.len() > 2 {
                f.finish_non_exhaustive()
            } else {
                f.finish()
            }
        }
    }

    let mut writer = ErrorTriggerWriter {
        error_on: "..\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", Foo(vec![1, 2, 3, 4])).is_err());
}

// Cover the try branches of
// * core::fmt::builders::DebugMap::<'a, 'b>::finish_non_exhaustive
// * core::fmt::builders::DebugMap::<'a, 'b>::key_with's try branches
// * core::fmt::builders::DebugMap::<'a, 'b>::value_with's try branches
#[test]
fn test_debug_map_finish_non_exhaustive_try_branches() {
    use std::fmt;

    // From the doctests
    struct Foo(Vec<(char, i32)>);

    impl fmt::Debug for Foo {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            // Print at most two elements, abbreviate the rest
            let mut f = fmt.debug_map();
            let f = f.entries(self.0.iter().take(2).map(|&(ref k, ref v)| (k, v)));
            if self.0.len() > 2 {
                f.finish_non_exhaustive()
            } else {
                f.finish()
            }
        }
    }

    let specimen = Foo(vec![('a', 1), ('b', 2), ('c', 3), ('d', 4)]);

    // finish_non_exhaustive
    let mut writer = ErrorTriggerWriter {
        error_on: "..\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "a",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "1",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ", ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ": ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ": ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: "\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let mut writer = ErrorTriggerWriter {
        error_on: ",\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());
}

// Covers the try branches of
// * core::fmt::builders::DebugTuple::<'a, 'b>::finish_non_exhaustive
// * core::fmt::builders::DebugTuple::<'a, 'b>::finish
// * core::fmt::builders::DebugTuple::<'a, 'b>::field_with
#[test]
fn test_debug_tuple_try_branches() {
    use std::fmt;

    // From the doctests
    struct Droop(i32, u64, #[allow(dead_code)] String);

    impl fmt::Debug for Droop {
        fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
            fmt.debug_tuple("Droop")
               .field(&self.0)
               .field(&self.1)
               .finish_non_exhaustive() // Show that some other field(s) exist.
        }
    }

    let specimen = Droop(10, 20, "secret!".to_owned());

    // finish_non_exhaustive: writer.write_str("..\n")?;
    let mut writer = ErrorTriggerWriter {
        error_on: "..\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    // field_with: self.fmt.write_str("(\n")?;
    let mut writer = ErrorTriggerWriter {
        error_on: "(\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    let multi_specimen = Droop(10, 20, "secret!".to_owned());

    // field_with: self.fmt.write_str(prefix)?;
    let mut writer = ErrorTriggerWriter {
        error_on: ", ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", multi_specimen).is_err());

    struct Floop(i32);

    impl fmt::Debug for Floop {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("")
                .field(&self.0)
                .finish()
        }
    }

    let mut writer = ErrorTriggerWriter {
        error_on: ",",
        allow_times: 0,
    };

    assert!(write!(writer, "{:?}", Floop(10)).is_err());
}

// Cover `core::fmt::builders::DebugStruct::<'a, 'b>::field_with`'s try branches
#[test]
fn test_debug_struct_field_with_try_branches() {
    #[derive(Debug)]
    #[allow(dead_code)]
    struct Fields {
        one: u64,
        two: u64
    }

    let specimen = Fields { one: 1, two: 2 };

    // self.fmt.write_str(" {\n")?;
    let mut writer = ErrorTriggerWriter {
        error_on: " {\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    // writer.write_str(": ")?;
    let mut writer = ErrorTriggerWriter {
        error_on: ": ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", specimen).is_err());

    // self.fmt.write_str(prefix)?;
    let mut writer = ErrorTriggerWriter {
        error_on: " { ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    // self.fmt.write_str(name)?;
    let mut writer = ErrorTriggerWriter {
        error_on: "one",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());

    // self.fmt.write_str(": ")?;
    let mut writer = ErrorTriggerWriter {
        error_on: ": ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", specimen).is_err());
}

// Cover `core::fmt::builders::DebugInner::<'a, 'b>::entry_with`'s try branches
#[test]
fn test_debug_inner_entry_with_try_branches() {
    struct Entry;

    impl fmt::Debug for Entry {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.write_str("Entry")
        }
    }

    // self.fmt.write_str("\n")?
    let mut writer = ErrorTriggerWriter {
        error_on: "\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", [Entry]).is_err());

    // The entry formatter itself: entry_fmt(&mut writer)?
    let mut writer = ErrorTriggerWriter {
        error_on: "Entry",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", [Entry]).is_err());

    // writer.write_str(",\n")?
    let mut writer = ErrorTriggerWriter {
        error_on: ",\n",
        allow_times: 0,
    };
    assert!(write!(writer, "{:#?}", [Entry]).is_err());

    // self.fmt.write_str(", ")?
    let mut writer = ErrorTriggerWriter {
        error_on: ", ",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", [Entry, Entry]).is_err());
}

// Covers `<core::str::lossy::Debug<'_> as core::fmt::Debug>::fmt`'s try branches
#[test]
fn test_debug_lossy_try_branches() {
    // f.write_char('"')?;
    let mut writer = ErrorTriggerWriter {
        error_on: "\"",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"hello".utf8_chunks().debug()).is_err());

    // f.write_str(&valid[from..i])?;
    let mut writer = ErrorTriggerWriter {
        error_on: "abc",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"abc\n".utf8_chunks().debug()).is_err());

    // f.write_char(c)?;
    let mut writer = ErrorTriggerWriter {
        error_on: "\\",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"\n".utf8_chunks().debug()).is_err());

    // f.write_str(&valid[from..])?;
    let mut writer = ErrorTriggerWriter {
        error_on: "\\x",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"\xFF".utf8_chunks().debug()).is_err());

    // f.write_str(&valid[from..])?;
    let mut writer = ErrorTriggerWriter {
        error_on: "abc",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"\nabc".utf8_chunks().debug()).is_err());

    // write!(f, "\\x{:02X}", b)?;
    let mut writer = ErrorTriggerWriter {
        error_on: "\\x",
        allow_times: 0,
    };
    assert!(write!(writer, "{:?}", b"\xFF".utf8_chunks().debug()).is_err());
}
