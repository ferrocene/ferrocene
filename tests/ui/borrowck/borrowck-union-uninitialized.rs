struct S {
    a: u8,
}

union U {
    a: u8,
}

fn main() {
    unsafe {
        let mut s: S;
        let mut u: U;
        s.a = 0; //~ ERROR E0381
        u.a = 0; //~ ERROR E0381
        let sa = s.a;
        let ua = u.a;
    }
}

//
// ferrocene-annotations: fls_3xvm61x0t251
// Initialization
//
// ferrocene-annotations: fls_fmdn7n7s413d
// Union Type
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
