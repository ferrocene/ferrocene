//@ run-pass

pub fn main() {
    'foo: loop {
        loop {
            break 'foo;
        }
    }

    'bar: for _ in 0..100 {
        loop {
            break 'bar;
        }
    }

    'foobar: while 1 + 1 == 2 {
        loop {
            break 'foobar;
        }
    }
}

// ferrocene-annotations: fls_onfyolkcbeh3
// For Loops
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
