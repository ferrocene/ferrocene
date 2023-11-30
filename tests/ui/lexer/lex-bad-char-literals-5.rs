static c: char = '\x10\x10';
//~^ ERROR: character literal may only contain one codepoint

fn main() {
    let ch: &str = '\x10\x10';
    //~^ ERROR: character literal may only contain one codepoint
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
