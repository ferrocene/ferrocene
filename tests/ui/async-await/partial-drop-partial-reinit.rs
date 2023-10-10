// edition:2021
// revisions: no_drop_tracking drop_tracking
// [drop_tracking] compile-flags: -Zdrop-tracking=yes
// [no_drop_tracking] compile-flags: -Zdrop-tracking=no
#![feature(negative_impls)]
#![allow(unused)]

fn main() {
    gimme_send(foo());
    //~^ ERROR cannot be sent between threads safely
    //~| NOTE cannot be sent
    //~| NOTE bound introduced by
    //~| NOTE appears within the type
    //~| NOTE captures the following types
}

fn gimme_send<T: Send>(t: T) {
    //~^ NOTE required by this bound
    //~| NOTE required by a bound
    drop(t);
}

struct NotSend {}

impl Drop for NotSend {
    fn drop(&mut self) {}
}

impl !Send for NotSend {}

async fn foo() {
    //~^ NOTE used within this `async fn` body
    //~| NOTE within this `impl Future
    let mut x = (NotSend {},);
    drop(x.0);
    x.0 = NotSend {};
    bar().await;
}

async fn bar() {}
