//@ run-pass
//@ revisions: default mir-opt
//@[mir-opt] compile-flags: -Zmir-opt-level=4

use std::panic::Location;

const LOCATION: &Location = Location::caller();

const TRACKED: &Location = tracked();
#[track_caller]
const fn tracked() -> &'static Location<'static> {
    Location::caller()
}

const NESTED: &Location = nested_location();
const fn nested_location() -> &'static Location<'static> {
    Location::caller()
}

const CONTAINED: &Location = contained();
const fn contained() -> &'static Location<'static> {
    tracked()
}

fn main() {
    assert_eq!(LOCATION.file(), file!());
    assert_eq!(LOCATION.line(), 7);
    assert_eq!(LOCATION.column(), 29);

    assert_eq!(TRACKED.file(), file!());
    assert_eq!(TRACKED.line(), 9);
    assert_eq!(TRACKED.column(), 28);

    assert_eq!(NESTED.file(), file!());
    assert_eq!(NESTED.line(), 17);
    assert_eq!(NESTED.column(), 5);

    assert_eq!(CONTAINED.file(), file!());
    assert_eq!(CONTAINED.line(), 22);
    assert_eq!(CONTAINED.column(), 5);
}

// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
// ferrocene-annotations: fls_xa4nbfas01cj
// Call Expressions
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
// ferrocene-annotations: fls_wjldgtio5o75
// Macro Expansion
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
