//@ run-fail
//@ error-pattern:quux
//@ ignore-emscripten no processes

fn my_err(s: String) -> ! {
    println!("{}", s);
    panic!("quux");
}

fn main() {
    3_usize == my_err("bye".to_string());
}

// ferrocene-annotations: fls_nsvzzbldhq53
// Comparison Expressions

// ferrocene-annotations: fls_98lnexk53ru4
// Never Type
//
// ferrocene-annotations: fls_k02nt1m5fq1z
// Panic
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
