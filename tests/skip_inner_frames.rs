extern crate backtrace;

use backtrace::Backtrace;

const FRAME_RANGE: usize = 128; // should be close enough not to give false positives

// FIXME: on Windows 32-bit ('i686-pc-windows-msvc') backtraces contain some spurious calls 
// which are not in the code (for instance calls to RtlFindCharInUnicodeString), however generated 
// backtraces are consistent between runs (so probably this is not an issue with synchronization?).
// Until resolved those test are ignored and `Backtrace::ext_index()` always returns None.
#[test]
#[cfg_attr(not(all(target_os = "windows", target_arch = "x86")), ignore)]
fn ext_index_must_be_0_on_win32() {
    let b = Backtrace::new();
    assert_eq!(b.ext_index(), 0);
}

#[test]
#[cfg_attr(any(not(any(feature = "libunwind", feature = "unix-backtrace", feature = "dbghelp")), all(target_os = "windows", target_arch = "x86")), ignore)]
fn backtrace_new_unresolved_should_start_with_call_site_trace() {
    let mut b = Backtrace::new_unresolved();
    b.resolve();
	println!("{:?}", b);
	println!("{:#?}", b);

    assert!(!b.frames().is_empty());
    assert!(b.ext_index() > 0);

    let this_ip = backtrace_new_unresolved_should_start_with_call_site_trace as usize;
    let frame_ip = b.ext_frames().first().unwrap().ip() as usize;

    assert!(frame_ip >= this_ip);
    assert!(frame_ip <= this_ip + FRAME_RANGE);
}

#[test]
#[cfg_attr(any(not(any(feature = "libunwind", feature = "unix-backtrace", feature = "dbghelp")), all(target_os = "windows", target_arch = "x86")), ignore)]
fn backtrace_new_should_start_with_call_site_trace() {
    let b = Backtrace::new();
    println!("{:?}", b);

    assert!(!b.frames().is_empty());
    assert!(b.ext_index() > 0);

    let this_ip = backtrace_new_should_start_with_call_site_trace as usize;
    let frame_ip = b.ext_frames().first().unwrap().ip() as usize;

    assert!(frame_ip >= this_ip);
    assert!(frame_ip <= this_ip + FRAME_RANGE);
}