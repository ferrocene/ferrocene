//@ run-pass

#![allow(dead_code)]

struct ArrayStruct<T, const N: usize> {
    data: [T; N],
}

struct ArrayTuple<T, const N: usize>([T; N]);

fn main() {
    let _ = ArrayStruct { data: [0u32; 8] };
    let _ = ArrayTuple([0u32; 8]);
}

// ferrocene-annotations: fls_lv7w7aalpwm5
// Type Inference
