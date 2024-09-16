// Ensures that all `fn` forms can have all the function qualifiers syntactically.

//@ edition:2018

fn main() {
    async fn ff1() {} // OK.
    unsafe fn ff2() {} // OK.
    const fn ff3() {} // OK.
    extern "C" fn ff4() {} // OK.
    const async unsafe extern "C" fn ff5() {}
    //~^ ERROR functions cannot be both `const` and `async`

    trait X {
        async fn ft1(); // OK.
        unsafe fn ft2(); // OK.
        const fn ft3(); //~ ERROR functions in traits cannot be declared const
        extern "C" fn ft4(); // OK.
        const async unsafe extern "C" fn ft5();
        //~^ ERROR functions in traits cannot be declared const
        //~| ERROR functions cannot be both `const` and `async`
    }

    struct Y;
    impl X for Y {
        async fn ft1() {} // OK.
        unsafe fn ft2() {} // OK.
        const fn ft3() {} //~ ERROR functions in trait impls cannot be declared const
        extern "C" fn ft4() {}
        const async unsafe extern "C" fn ft5() {}
        //~^ ERROR functions in trait impls cannot be declared const
        //~| ERROR functions cannot be both `const` and `async`
    }

    impl Y {
        async fn fi1() {} // OK.
        unsafe fn fi2() {} // OK.
        const fn fi3() {} // OK.
        extern "C" fn fi4() {} // OK.
        const async unsafe extern "C" fn fi5() {}
        //~^ ERROR functions cannot be both `const` and `async`
    }

    extern "C" {
        async fn fe1(); //~ ERROR functions in `extern` blocks cannot have qualifiers
        unsafe fn fe2(); //~ ERROR items in unadorned `extern` blocks cannot have safety qualifiers
        const fn fe3(); //~ ERROR functions in `extern` blocks cannot have qualifiers
        extern "C" fn fe4(); //~ ERROR functions in `extern` blocks cannot have qualifiers
        const async unsafe extern "C" fn fe5();
        //~^ ERROR functions in `extern` blocks
        //~| ERROR functions in `extern` blocks
        //~| ERROR functions in `extern` blocks
        //~| ERROR functions cannot be both `const` and `async`
        //~| ERROR items in unadorned `extern` blocks cannot have safety qualifiers
    }
}

// ferrocene-annotations: fls_qcb1n9c0e5hz
// Functions
