//@ check-pass

fn main() {
    if let Some([b'@', filename @ ..]) = Some(b"@abc123") {
        println!("filename {:?}", filename);
    }
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
