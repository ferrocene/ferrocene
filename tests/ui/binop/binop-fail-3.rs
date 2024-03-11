//@ run-fail
//@ error-pattern:quux
//@ ignore-emscripten no processes

fn foo() -> ! {
    panic!("quux");
}

fn main() {
    foo() == foo(); // these types wind up being defaulted to ()
}

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
//
// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions
