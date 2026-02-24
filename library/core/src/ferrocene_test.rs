use crate::ops::IndexRange;
use crate::slice::SliceIndex;

pub fn test_index_range_slice_index() {
    let mut slice_bytes = [1, 2, 3, 4, 5];
    let res = [1, 2].as_slice();

    let range_some = IndexRange::zero_to(2);
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    {
        let slice_ref = slice_bytes.as_slice();

        assert_eq!(Some(res), SliceIndex::get(range_some.clone(), slice_ref));
        assert_eq!(None, SliceIndex::get(range_none.clone(), slice_ref));
        assert_eq!(res, SliceIndex::index(range_some.clone(), slice_ref));
    }

    {
        let slice_mut = slice_bytes.as_mut_slice();

        assert_eq!(Some(res), SliceIndex::get_mut(range_some.clone(), slice_mut).as_deref());
        assert_eq!(None, SliceIndex::get_mut(range_none.clone(), slice_mut));
        assert_eq!(res, SliceIndex::index_mut(range_some.clone(), slice_mut));
    }
}

pub fn test_index_range_slice_index_panic() {
    let slice_bytes = [1, 2, 3, 4, 5];
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    let slice_ref = slice_bytes.as_slice();
    SliceIndex::index(range_none.clone(), slice_ref);
}

pub fn test_index_range_slice_index_panic_mut() {
    let mut slice_bytes = [1, 2, 3, 4, 5];
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    let slice_mut = slice_bytes.as_mut_slice();
    SliceIndex::index_mut(range_none.clone(), slice_mut);
}

pub fn test_wrapped_call_once() {
    let wrapped = core::ops::NeverShortCircuit::wrap_mut_1(|c| c + 10);
    assert_eq!(25, wrapped.call_once((15,)).0);
}

pub fn test_index_range_iterator() {
    let mut ir = IndexRange::zero_to(5);

    // size_hint
    assert_eq!((5, Some(5)), ir.size_hint());

    // advance_by
    assert_eq!(Ok(()), ir.advance_by(2));
    assert_eq!(2, ir.start());

    // fold
    assert_eq!((), ir.fold((), |_, _| ()));
}

pub fn test_index_range_double_ended_iterator() {
    let mut ir = IndexRange::zero_to(5);

    // advance_back_by
    assert_eq!(Ok(()), ir.advance_back_by(2));
    assert_eq!(3, ir.end());

    // rfold
    assert_eq!((), ir.rfold((), |_, _| ()));
}

/// Provides println-like functions for debugging libcore.
///
/// This is useful because within libcore there are no `println!` or `dbg!` macros available.
///
/// If building the subset targets, as is done when executing `./x test library/core --no-doc --coverage=library`, it is necessary to mark those function calls with `#[cfg(not(feature = "ferrocene_subset"))]`.
///
/// ```
/// use crate::ferrocene_test::println;
///
/// fn some_libcore_function(param_a: u32) {
///     #[cfg(not(feature = "ferrocene_subset"))]
///     println::eprintln_args(format_args!("param_a={}", param_a));
///
///     if param_a > 10 {
///         #[cfg(not(feature = "ferrocene_subset"))]
///         println::eprintln_str("hit branch A");
///     } else {
///         #[cfg(not(feature = "ferrocene_subset"))]
///         println::eprintln_str("hit branch B");
///     }
/// }
/// ```
#[cfg(any(target_os = "linux", target_os = "macos"))]
pub(crate) mod println {
    use crate::fmt;

    unsafe extern "C" {
        fn dprintf(fd: i32, s: *const u8, ...);
    }

    pub fn eprintln_str(s: &str) {
        eprintln_args(format_args!("{s}"));
    }

    pub fn eprintln_args(args: fmt::Arguments<'_>) {
        let mut buf = Buffer::new();
        let mut f = fmt::Formatter::new(&mut buf, fmt::FormattingOptions::new());
        fmt::Display::fmt(&format_args!("{args}\n"), &mut f).unwrap();
        // SAFETY: the arguments to the function are correct
        unsafe {
            dprintf(2, "%s\0".as_ptr(), buf.read().as_ptr() as *const u8);
        }
    }

    struct Buffer {
        buf: [u8; 10 * 1024],
        i: usize,
    }

    impl Buffer {
        fn new() -> Self {
            Self { buf: [0; _], i: 0 }
        }

        fn read(&self) -> &[u8] {
            &self.buf[..self.i]
        }
    }

    impl fmt::Write for Buffer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            for byte in s.as_bytes() {
                self.buf[self.i] = *byte;
                self.i += 1;
            }
            Ok(())
        }
    }
}
