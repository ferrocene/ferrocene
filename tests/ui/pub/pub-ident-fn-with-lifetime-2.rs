pub   bar<'a>(&self, _s: &'a usize) -> bool { true }
//~^ ERROR missing `fn` for method definition

fn main() {
    bar(2);
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
