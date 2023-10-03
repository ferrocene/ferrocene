macro_rules! baz(
    baz => () //~ ERROR invalid macro matcher;
);

fn main() {
    baz!(baz);
}

// ferrocene-annotations: fls_xa7lp0zg1ol2
// Declarative Macros
//
// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
