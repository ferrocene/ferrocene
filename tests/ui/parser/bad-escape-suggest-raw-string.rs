fn main() {
    let ok = r"ab\[c";
    let bad = "ab\[c";
    //~^ ERROR unknown character escape: `[`
    //~| HELP for more information, visit <https://doc.rust-lang.org/reference/tokens.html#literals>
    //~| HELP if you meant to write a literal backslash (perhaps escaping in a regular expression), consider a raw string literal
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
//
// ferrocene-annotations: fls_usr6iuwpwqqh
// Raw String Literals
//
// ferrocene-annotations: fls_hucd52suu6it
// Simple String Literals
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
//
// ferrocene-annotations: fls_h0dvogc64tfh
// Literal Expressions
