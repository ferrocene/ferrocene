struct Qux(i32);

fn bad() {
    let mut map = std::collections::HashMap::new();
    map.insert(('a', 'b'), ('c', 'd'));

    for ((_, _), (&mut c, _)) in &mut map {
    //~^ ERROR mismatched types
        if c == 'e' {}
    }
}

fn bad2() {
    for Some(Qux(_)) | None in [Some(""), None] {
    //~^ ERROR mismatched types
        todo!();
    }
}

fn main() {}

// ferrocene-annotations: fls_rce8bb7nz2jy
// Tuple Pattern Matching
//
// ferrocene-annotations: fls_urbr5rg9206v
// Tuple Patterns
//
// ferrocene-annotations: fls_yc4xm4hrfyw7
// Underscore Pattern Matching
//
// ferrocene-annotations: fls_qfsfnql1t7m
// Underscore Patterns
