//@ check-pass
//@ check-stdout
//@ compile-flags: -V
// ignore-tidy-linelength
//@ normalize-stdout: "rustc .*(Ferrocene .*)" -> "rustc UPSTREAM_VERSION (Ferrocene FERROCENE_VERSION)"
//@ regex-error-pattern: rustc .*(Ferrocene .*)
