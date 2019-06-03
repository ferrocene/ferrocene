#[test]
fn all_frames_have_symbols() {
    println!("{:?}", backtrace::Backtrace::new());

    let mut all_have_symbols = true;
    backtrace::trace(|frame| {
        let mut any = false;
        backtrace::resolve_frame(frame, |sym| {
            if sym.name().is_some() {
                any = true;
            }
        });
        if !any && !frame.ip().is_null() {
            all_have_symbols = false;
        }
        true
    });
    assert!(all_have_symbols);
}
