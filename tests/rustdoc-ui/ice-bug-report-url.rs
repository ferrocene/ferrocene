//@ compile-flags: -Ztreat-err-as-bug
//@ rustc-env:RUSTC_ICE=0
//@ failure-status: 101

//@ normalize-stderr: "note: compiler flags.*\n\n" -> ""
//@ normalize-stderr: "note: rustc.*running on.*" -> "note: rustc {version} running on {platform}"
//@ normalize-stderr: "thread.*panicked at .*" -> ""
//@ normalize-stderr: " +\d{1,}: .*\n" -> ""
//@ normalize-stderr: " + at .*\n" -> ""
//@ normalize-stderr: ".*note: Some details are omitted.*\n" -> ""

fn wrong()
//~^ ERROR expected one of

//~? RAW aborting due to
//~? RAW we would appreciate a bug report: https://github.com/ferrocene/ferrocene/issues/new
