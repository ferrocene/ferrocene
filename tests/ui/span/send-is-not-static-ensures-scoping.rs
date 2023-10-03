struct Guard<'a> {
    f: Box<dyn Fn() + Send + 'a>,
}

fn scoped<'a, F: Fn() + Send + 'a>(f: F) -> Guard<'a> {
    Guard { f: Box::new(f) }
}

impl<'a> Guard<'a> {
    fn join(self) {}
}

fn main() {
    let bad = {
        let x = 1;
        let y = &x;
        //~^ ERROR `x` does not live long enough

        scoped(|| {
            let _z = y;
            //~^ ERROR `y` does not live long enough
        })
    };

    bad.join();
}

// ferrocene-annotations: fls_9ucqbbd0s2yo
// Struct Types
//
// ferrocene-annotations: fls_vhpwge5123cm
// Generic Parameters
//
// ferrocene-annotations: fls_yqcygq3y6m5j
// Lifetimes
//
// ferrocene-annotations: fls_qa98qdi42orq
// Trait Object Types
//
// ferrocene-annotations: fls_fk2m2irwpeof
// Implementations
//
// ferrocene-annotations: fls_a14slch83hzn
// Borrowing
//
// ferrocene-annotations: fls_qztk0bkju9u
// Borrow Expression
//
// ferrocene-annotations: fls_svkx6szhr472
// Ownership
//
// ferrocene-annotations: fls_eiw4by8z75di
// Send and Sync
