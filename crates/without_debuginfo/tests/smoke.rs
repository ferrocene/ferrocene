#[test]
fn all_frames_have_symbols() {
    println!("{:?}", backtrace::Backtrace::new());

    let mut missing_symbols = 0;
    let mut has_symbols = 0;
    backtrace::trace(|frame| {
        let mut any = false;
        backtrace::resolve_frame(frame, |sym| {
            if sym.name().is_some() {
                any = true;
            }
        });
        if any {
            has_symbols += 1;
        } else if !frame.ip().is_null() {
            missing_symbols += 1;
        }
        true
    });

    // FIXME(#346) currently on MinGW we can't symbolize kernel32.dll and other
    // system libraries, which means we miss the last few symbols.
    if cfg!(windows) && cfg!(target_env = "gnu") {
        assert!(missing_symbols < has_symbols && has_symbols > 4);
    } else {
        assert_eq!(missing_symbols, 0);
    }
}
