macro_rules! funny {
    ($a:expr, $b:ident) => {
        match [1, 2] {
            [$a, $b] => {}
        }
    };
}

fn main() {
    funny!(a, a);
    //~^ ERROR cannot find value `a` in this scope
    //~| ERROR arbitrary expressions aren't allowed in patterns
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_xlfo7di0gsqz
// Hygiene
