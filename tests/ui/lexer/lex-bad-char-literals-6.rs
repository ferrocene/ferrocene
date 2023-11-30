fn main() {
    let x: &str = 'ab';
    //~^ ERROR: character literal may only contain one codepoint
    let y: char = 'cd';
    //~^ ERROR: character literal may only contain one codepoint
    let z = 'ef';
    //~^ ERROR: character literal may only contain one codepoint

    if x == y {}
    if y == z {}  // no error here
    if x == z {}

    let a: usize = "";
    //~^ ERROR: mismatched types
}

// ferrocene-annotations: fls_ypa86oqxhn9u
// Character Literals
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
