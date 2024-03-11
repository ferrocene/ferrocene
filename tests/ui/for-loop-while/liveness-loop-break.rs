//@ run-pass
fn test() {
    let v;
    loop {
        v = 3;
        break;
    }
    println!("{}", v);
}

pub fn main() {
    test();
}

// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
