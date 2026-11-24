fn main() {
    let _ = ("a̐éö̲", 0u7); //~ ERROR invalid width
    let _ = ("아あ", 1i42); //~ ERROR invalid width
    let _ = a̐é; //~ ERROR cannot find
    //~^ WARN identifier contains a character that is for non-linguistic, specialized usage
}

// ferrocene-annotations: fls_2i089jvv8j5g
// Character Set
//
// ferrocene-annotations: fls_boyhlu5srp6u
// String Literals
