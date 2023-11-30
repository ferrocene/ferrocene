macro_rules! foo { ($($tt:tt)*) => () }

fn main() {
    foo! {
        bar, "baz", 1, 2.0
    ) //~ ERROR mismatched closing delimiter
}

// ferrocene-annotations: fls_vnvt40pa48n8
// Macro Invocation
