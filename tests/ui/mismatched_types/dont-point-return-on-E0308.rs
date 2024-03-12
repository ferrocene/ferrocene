//@ edition:2021

async fn f(_: &()) {}
//~^ NOTE function defined here
//~| NOTE
// Second note is the span of the underlined argument, I think...

fn main() {
    (|| async {
        Err::<(), ()>(())?;
        f(());
        //~^ ERROR mismatched types
        //~| NOTE arguments to this function are incorrect
        //~| NOTE expected `&()`, found `()`
        //~| HELP consider borrowing here
        Ok::<(), ()>(())
    })();
}

// ferrocene-annotations: fls_tjyexqrx0fx5
// Closure Expressions
//
// ferrocene-annotations: fls_aadan19t5006
// Async Blocks
