//@ run-pass

fn main() {
    let mut x: &[_] = &[1, 2, 3, 4];

    let mut result = vec![];
    loop {
        x = match *x {
            [1, n, 3, ref rest @ ..] => {
                result.push(n);
                rest
            }
            [n, ref rest @ ..] => {
                result.push(n);
                rest
            }
            [] =>
                break
        }
    }
    assert_eq!(result, [2, 4]);
}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
