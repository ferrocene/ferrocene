//@ known-bug: #140011
//@compile-flags: -Wrust-2021-incompatible-closure-captures
//@ edition: 2015
enum b {
    c(d),
    e(f),
}
struct f;
fn g() {
    let h;
    || b::e(a) = h;
}
