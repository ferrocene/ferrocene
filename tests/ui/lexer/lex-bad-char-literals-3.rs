static c: char = '●●';
//~^ ERROR: character literal may only contain one codepoint

fn main() {
    let ch: &str = '●●';
    //~^ ERROR: character literal may only contain one codepoint
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
