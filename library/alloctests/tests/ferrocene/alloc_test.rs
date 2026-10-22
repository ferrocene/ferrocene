use core::alloc::Layout;

#[test]
fn test_realloc() {
    let size_1 = 1024;
    let size_2 = 2048;

    unsafe {
        // allocate a block of memory
        let layout = Layout::from_size_align(size_1, 1).unwrap();
        let ptr = alloc::alloc::alloc(layout.clone());

        // read the first 10 bytes, before realloc
        let before = core::slice::from_raw_parts(ptr, 10).to_vec();

        // reallocate to grow the allocation
        let ptr = alloc::alloc::realloc(ptr, layout, size_2);

        // read the first 10 bytes, after realloc
        let after = core::slice::from_raw_parts(ptr, 10).to_vec();

        // deallocate the memory
        let layout = Layout::from_size_align(size_2, 1).unwrap();
        alloc::alloc::dealloc(ptr, layout);

        // ensure the first ten bytes are the same before and after realloc
        assert_eq!(before, after);
    }
}
