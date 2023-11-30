// This test needs to the last one appearing in this file as it kills the parser
static c: char =
    'nope' //~ ERROR: character literal may only contain one codepoint
;

fn main() {}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
