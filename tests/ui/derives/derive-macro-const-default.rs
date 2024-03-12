//@ check-pass
#[derive(Clone, PartialEq, Debug)]
struct Example<T, const N: usize = 1usize>([T; N]);

fn main() {
    let a = Example([(); 16]);
    let b = a.clone();
    if a != b {
        let _c = format!("{:?}", a);
    }
}

//
// ferrocene-annotations: fls_z7q8kbjwdc7g
// Method Call Expressions
//
// ferrocene-annotations: fls_wqazkzle0ix9
// Method Resolution
