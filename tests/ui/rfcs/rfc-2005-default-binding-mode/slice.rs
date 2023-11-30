// run-pass

fn slice_pat() {
    let sl: &[u8] = b"foo";

    match sl {
        [first, remainder @ ..] => {
            let _: &u8 = first;
            assert_eq!(first, &b'f');
            assert_eq!(remainder, b"oo");
        }
        [] => panic!(),
    }
}

fn slice_pat_omission() {
     match &[0, 1, 2] {
        [..] => {}
     };
}

fn main() {
    slice_pat();
    slice_pat_omission();
}

// ferrocene-annotations: fls_vnai6ag4qrdb
// Identifier Pattern Matching
//
// ferrocene-annotations: fls_7bxv8lybxm18
// Identifier Patterns
//
// ferrocene-annotations: fls_7wpgnp4kjq82
// Rest Patterns
//
// ferrocene-annotations: fls_57ic33pwdvp3
// Slice Pattern Matching
//
// ferrocene-annotations: fls_qte70mgzpras
// Slice Patterns
