extern crate backtrace;

use backtrace::Backtrace;

const FRAME_RANGE: usize = 128; // should be close enough not to give false positives

#[test]
#[cfg_attr(any(not(any(feature = "libunwind", feature = "unix-backtrace", feature = "dbghelp")), all(target_os = "windows", target_arch = "x86")), ignore)]
fn backtrace_new_unresolved_should_start_with_call_site_trace() {
    let mut b = Backtrace::new_unresolved();
    b.resolve();
	println!("{:?}", b);
	println!("{:#?}", b);

    assert!(!b.frames().is_empty());

    let this_ip = backtrace_new_unresolved_should_start_with_call_site_trace as usize;
    let frame_ip = b.frames().first().unwrap().ip() as usize;

    assert!(frame_ip >= this_ip);
    assert!(frame_ip <= this_ip + FRAME_RANGE);
}

#[test]
#[cfg_attr(any(not(any(feature = "libunwind", feature = "unix-backtrace", feature = "dbghelp")), all(target_os = "windows", target_arch = "x86")), ignore)]
fn backtrace_new_should_start_with_call_site_trace() {
    let b = Backtrace::new();
    println!("{:?}", b);

    assert!(!b.frames().is_empty());

    let this_ip = backtrace_new_should_start_with_call_site_trace as usize;
    let frame_ip = b.frames().first().unwrap().ip() as usize;

    assert!(frame_ip >= this_ip);
    assert!(frame_ip <= this_ip + FRAME_RANGE);
}
