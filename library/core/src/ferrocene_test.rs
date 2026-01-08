use crate::ops::IndexRange;
use crate::slice::SliceIndex;

pub fn test_index_range_slice_index() {
    let mut slice_bytes = [1, 2, 3, 4, 5];
    let res = [1, 2].as_slice();

    let range_some = IndexRange::zero_to(2);
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    {
        let slice_ref = slice_bytes.as_slice();

        assert_eq!(Some(res), SliceIndex::get(range_some.clone(), slice_ref));
        assert_eq!(None, SliceIndex::get(range_none.clone(), slice_ref));
        assert_eq!(res, SliceIndex::index(range_some.clone(), slice_ref));
    }

    {
        let slice_mut = slice_bytes.as_mut_slice();

        assert_eq!(Some(res), SliceIndex::get_mut(range_some.clone(), slice_mut).as_deref());
        assert_eq!(None, SliceIndex::get_mut(range_none.clone(), slice_mut));
        assert_eq!(res, SliceIndex::index_mut(range_some.clone(), slice_mut));
    }
}

pub fn test_index_range_slice_index_panic() {
    let slice_bytes = [1, 2, 3, 4, 5];
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    let slice_ref = slice_bytes.as_slice();
    SliceIndex::index(range_none.clone(), slice_ref);
}

pub fn test_index_range_slice_index_panic_mut() {
    let mut slice_bytes = [1, 2, 3, 4, 5];
    // SAFETY: start==end
    let range_none = unsafe { IndexRange::new_unchecked(100, 100) };

    let slice_mut = slice_bytes.as_mut_slice();
    SliceIndex::index_mut(range_none.clone(), slice_mut);
}
