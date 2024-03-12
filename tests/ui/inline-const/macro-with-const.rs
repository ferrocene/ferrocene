//@ check-pass

macro_rules! exp {
    (const $n:expr) => {
        $n
    };
}

macro_rules! stmt {
    (exp $e:expr) => {
        $e
    };
    (exp $($t:tt)+) => {
        exp!($($t)+)
    };
}

fn main() {
    stmt!(exp const 1);
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_ixjc5jaamx84
// Constants
//
// ferrocene-annotations: fls_9kjpxri0axvg
// Weak Keywords
