//@ edition:2018

extern {
    async fn L() { //~ ERROR: incorrect function inside `extern` block
        //~^ ERROR: functions in `extern` blocks cannot have `async` qualifier
        async fn M() {}
    }
}

fn main() {}

// ferrocene-annotations: fls_yztwtek0y34v
// External Functions
