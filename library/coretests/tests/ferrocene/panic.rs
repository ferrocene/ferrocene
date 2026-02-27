#[test]
fn panic_location() {
    let loc = core::panic::Location::caller();

    let _ = loc.line();
    let _ = loc.column();
}
