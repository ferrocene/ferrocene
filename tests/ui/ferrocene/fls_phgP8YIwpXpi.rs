// This is for these FLS rules
// - fls_ftE05Blr5Esv
// - fls_csdjfwczlzfd

//@ run-pass

fn main() {
    let list = Vec::from([1, 2, 3]);
    // place references in a block to ensure they are dropped before original data
    {
        let list_ref = &list;
        let another_list_ref = list_ref;
        // demo that the reference (still) points to the original data
        assert!(core::ptr::eq(list_ref, &list));
        // demo that the other reference points to the original data
        assert!(core::ptr::eq(another_list_ref, &list));
    }
    // demo that original data remains
    assert_eq!(list, [1, 2, 3]);
}

// ferrocene-annotations: fls_phgp8yiwpxpi
// References
