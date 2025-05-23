//@ known-bug: #134335
//@compile-flags: -Zunstable-options --crate-type=lib
//@ edition: 2024
pub async fn async_closure(x: &mut i32) {
    let c = async move || {
        *x += 1;
    };
    call_once(c).await;
}

fn call_once<T>(f: impl FnOnce() -> T) -> T {
    f()
}
