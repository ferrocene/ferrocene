fn main() {
    1 + Some(1); //~ ERROR cannot add `Option<{integer}>` to `{integer}`
    2 as usize - Some(1); //~ ERROR cannot subtract `Option<{integer}>` from `usize`
    3 * (); //~ ERROR cannot multiply `{integer}` by `()`
    4 / ""; //~ ERROR cannot divide `{integer}` by `&str`
    5 < String::new(); //~ ERROR can't compare `{integer}` with `String`
    6 == Ok(1); //~ ERROR can't compare `{integer}` with `Result<{integer}, _>`
}

// ferrocene-annotations: fls_1k9mkv7rbezi
// Arithmetic Expressions
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
//
// ferrocene-annotations: fls_1qhsun1vyarz
// Type Cast Expressions
//
// ferrocene-annotations: fls_zfibijmf8qe1
// Arithmetic Overflow
