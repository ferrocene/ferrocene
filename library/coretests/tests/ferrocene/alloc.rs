use core::alloc::{GlobalAlloc, Layout};
use core::cell::UnsafeCell;
use core::ptr::null_mut;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::Relaxed;

// Covers `core::alloc::layout::Layout::is_size_align_valid`'s None
#[test]
fn test_layout_is_size_align_valid_none() {
    assert!(Layout::from_size_align(0, 0).is_err());
    assert!(Layout::from_size_align(0, 3).is_err());
}

// SimpleAllocator implementation copied from [core::alloc::GlobalAlloc] docs example
const ARENA_SIZE: usize = 128 * 1024;
const MAX_SUPPORTED_ALIGN: usize = 4096;
#[repr(C, align(4096))] // 4096 == MAX_SUPPORTED_ALIGN
struct SimpleAllocator {
    arena: UnsafeCell<[u8; ARENA_SIZE]>,
    remaining: AtomicUsize, // we allocate from the top, counting down
}

impl SimpleAllocator {
    fn new() -> Self {
        SimpleAllocator {
            arena: UnsafeCell::new([0x55; ARENA_SIZE]),
            remaining: AtomicUsize::new(ARENA_SIZE),
        }
    }
}

unsafe impl GlobalAlloc for SimpleAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let size = layout.size();
        let align = layout.align();

        // `Layout` contract forbids making a `Layout` with align=0, or align not power of 2.
        // So we can safely use a mask to ensure alignment without worrying about UB.
        let align_mask_to_round_down = !(align - 1);

        if align > MAX_SUPPORTED_ALIGN {
            return null_mut();
        }

        let mut allocated = 0;
        if self
            .remaining
            .try_update(Relaxed, Relaxed, |mut remaining| {
                if size > remaining {
                    return None;
                }
                remaining -= size;
                remaining &= align_mask_to_round_down;
                allocated = remaining;
                Some(remaining)
            })
            .is_err()
        {
            return null_mut();
        };
        unsafe { self.arena.get().cast::<u8>().add(allocated) }
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[test]
fn test_global_alloc_alloc_zeroed() {
    let simple_alloc = SimpleAllocator::new();
    let layout = Layout::from_size_align(256, 8).unwrap();

    let ptr = unsafe { simple_alloc.alloc_zeroed(layout) };

    let data = unsafe { core::slice::from_raw_parts(ptr, layout.size()) };
    assert!(data.iter().all(|byte| *byte == 0));
}

#[test]
fn test_global_alloc_alloc_zeroed_alloc_null() {
    let simple_alloc = SimpleAllocator::new();
    let layout = Layout::from_size_align(256, align_too_big()).unwrap();

    let ptr = unsafe { simple_alloc.alloc_zeroed(layout) };
    assert!(ptr.is_null());
}

#[test]
fn test_global_alloc_realloc() {
    let simple_alloc = SimpleAllocator::new();
    let layout = Layout::from_size_align(256, 8).unwrap();

    let ptr = unsafe { simple_alloc.alloc_zeroed(layout) };
    unsafe { ptr.write_bytes(0xAA, layout.size()) };

    let new_size = 512;
    let new_ptr = unsafe { simple_alloc.realloc(ptr, layout, new_size) };

    let data = unsafe { core::slice::from_raw_parts(new_ptr, new_size) };
    assert!(data[..layout.size()].iter().all(|byte| *byte == 0xAA));
}

#[test]
fn test_global_alloc_realloc_null() {
    let simple_alloc = SimpleAllocator::new();
    let layout = Layout::from_size_align(256, align_too_big()).unwrap();

    let ptr = unsafe { simple_alloc.alloc_zeroed(layout) };
    assert!(ptr.is_null());

    let new_size = 512;
    let new_ptr = unsafe { simple_alloc.realloc(ptr, layout, new_size) };
    assert!(new_ptr.is_null());
}

const fn align_too_big() -> usize {
    (MAX_SUPPORTED_ALIGN + 1).next_power_of_two()
}
