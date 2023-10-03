extern "C" {
    fn a(&mut self) {
        //~^ ERROR incorrect function inside `extern` block
        //~| ERROR `self` parameter is only allowed in associated functions
        fn b(buf: &Self) {}
    }
}

fn main() {}

// ferrocene-annotations: fls_tmoh3y9oyqsy
// External Blocks
