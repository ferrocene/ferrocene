struct Foo {
    t: String
}

fn cond() -> bool { true }

fn foo<F>(_: F) where F: FnOnce() {}

fn main() {
    let pth = break; //~ ERROR: `break` outside of a loop
    if cond() { continue } //~ ERROR: `continue` outside of a loop

    while cond() {
        if cond() { break }
        if cond() { continue }
        foo(|| {
            if cond() { break } //~ ERROR: `break` inside of a closure
            if cond() { continue } //~ ERROR: `continue` inside of a closure
        })
    }

    let rs: Foo = Foo{t: pth};

    let unconstrained = break; //~ ERROR: `break` outside of a loop

    // This used to ICE because `target_id` passed to `check_expr_break` would be the closure and
    // not the `loop`, which failed in the call to `find_breakable`. (#65383)
    'lab: loop {
        || {
            break 'lab;
            //~^ ERROR use of unreachable label `'lab`
            //~| ERROR `break` inside of a closure
        };
    }
}

// ferrocene-annotations: fls_jr4tpuyksr75
// Break Expressions
//
// ferrocene-annotations: fls_sjwrlwvpulp
// Continue Expressions
//
// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_sf4qnd43z2wc
// Infinite Loops
//
// ferrocene-annotations: fls_uusi0zej55is
// Loop Labels
//
// ferrocene-annotations: fls_5jjm1kt43axd
// While Loops
//
// ferrocene-annotations: fls_zjoamsr3dbqk
// Diverging Expressions
