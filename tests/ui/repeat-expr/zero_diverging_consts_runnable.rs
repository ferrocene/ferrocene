// Check that repeating a diverging constant 0 times doesn't lead to unsoundness.
//
// Issue: <https://github.com/rust-lang/rust/issues/165323>
//
//@ run-fail
//@ check-run-results
//@ exec-env:RUST_BACKTRACE=0

// Ferrocene note: do we have wonky optimizations
// ... see also tests/codegen-llvm/issues/enumerate-index-le-len-no-panic-92174.rs
//@ ignore-ferrocene.facade

fn get(v: &[u32], i: usize) -> u32 {
    if i >= v.len() {
        let _: [!; 0] = [const { panic!() }; 0];
    }
    v[i]
}

fn main() {
    let v = vec![1, 2, 3];
    let i = std::hint::black_box(4); // 4, hidden from the optimizer
    println!("v[{i}] = {}", get(&v, i));
}
