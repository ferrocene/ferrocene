<<<PULL-UPSTREAM>>> file deleted upstream; move the Ferrocene annotations if any, and delete this file
#![crate_name="foo"]
#![allow(dead_code)]

// (#55495: The --error-format is to sidestep an issue in our test harness)
//@ compile-flags: -C opt-level=0 --error-format human -Z print-fuel=foo
//@ check-pass

struct S1(u8, u16, u8);
struct S2(u8, u16, u8);
struct S3(u8, u16, u8);

fn main() {
}

// ferrocene-annotations: um_rustc_error_format
