#![feature(rustc_attrs)]

extern //~ WARN missing_abi
    "C"suffix //~ ERROR suffixes on string literals are invalid
    fn foo() {}

extern //~ WARN missing_abi
    "C"suffix //~ ERROR suffixes on string literals are invalid
{}

fn main() {
    ""suffix; //~ ERROR suffixes on string literals are invalid
    b""suffix; //~ ERROR suffixes on byte string literals are invalid
    r#""#suffix; //~ ERROR suffixes on string literals are invalid
    br#""#suffix; //~ ERROR suffixes on byte string literals are invalid
    'a'suffix; //~ ERROR suffixes on char literals are invalid
    b'a'suffix; //~ ERROR suffixes on byte literals are invalid

    1234u1024; //~ ERROR invalid width `1024` for integer literal
    1234i1024; //~ ERROR invalid width `1024` for integer literal
    1234f1024; //~ ERROR invalid width `1024` for float literal
    1234.5f1024; //~ ERROR invalid width `1024` for float literal

    1234suffix; //~ ERROR invalid suffix `suffix` for number literal
    0b101suffix; //~ ERROR invalid suffix `suffix` for number literal
    1.0suffix; //~ ERROR invalid suffix `suffix` for float literal
    1.0e10suffix; //~ ERROR invalid suffix `suffix` for float literal
}

#[rustc_dummy = "string"suffix]
//~^ ERROR suffixes on string literals are invalid
fn f() {}

#[must_use = "string"suffix]
//~^ ERROR suffixes on string literals are invalid
//~| ERROR malformed `must_use` attribute input
fn g() {}

#[link(name = "string"suffix)]
//~^ ERROR suffixes on string literals are invalid
//~| ERROR malformed `link` attribute input
extern "C" {}

#[rustc_layout_scalar_valid_range_start(0suffix)]
//~^ ERROR invalid suffix `suffix` for number literal
//~| ERROR malformed `rustc_layout_scalar_valid_range_start` attribute input
struct S;

// ferrocene-annotations: fls_fqaffyrjob7v
// Byte String Literals
//
// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
//
// ferrocene-annotations: fls_29tlg1vyqay2
// Float Literals
//
// ferrocene-annotations: fls_2ed4axpsy9u0
// Integer Literals
//
// ferrocene-annotations: fls_jps9102q0qfi
// Raw Byte String Literals
//
// ferrocene-annotations: fls_usr6iuwpwqqh
// Raw String Literals
//
// ferrocene-annotations: fls_msbaxfc09vkk
// Simple Byte String Literals
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
//
// ferrocene-annotations: fls_e7zgqroy2qxn
// Value Expressions
