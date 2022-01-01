use std::alloc::{handle_alloc_error, GlobalAlloc, Layout, System};
use std::cell::UnsafeCell;
use std::ptr::null_mut;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/***

Copied from https://github.com/sheredom/bump_alloc

 */

fn align_to(size: usize, align: usize) -> usize {
    (size + align - 1) & !(align - 1)
}

struct Inner {
    offset: AtomicUsize,
    mmap: *mut u8,
    initializing: AtomicBool,
}

pub struct BumpAlloc {
    inner: UnsafeCell<Inner>,
    size: usize,
}

unsafe impl Sync for BumpAlloc {}

impl BumpAlloc {
    pub const fn new() -> BumpAlloc {
        BumpAlloc::with_size(1024 * 1024 * 1024) // Default to one gigabyte.
    }

    pub const fn with_size(size: usize) -> BumpAlloc {
        BumpAlloc {
            inner: UnsafeCell::new(Inner {
                initializing: AtomicBool::new(true),
                mmap: null_mut(),
                offset: AtomicUsize::new(0),
            }),
            size,
        }
    }
}

// type of the size parameter to VirtualAlloc
#[cfg(all(windows, target_pointer_width = "32"))]
type WindowsSize = u32;
#[cfg(all(windows, target_pointer_width = "64"))]
type WindowsSize = u64;

unsafe impl GlobalAlloc for BumpAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let inner = &mut *self.inner.get();

        // If initializing is true it means we need to do the original mmap.
        if inner.initializing.swap(false, Ordering::Relaxed) {
            inner.mmap = System.alloc(Layout::from_size_align(self.size, 1).unwrap());

            if (*inner.mmap as isize) == -1isize {
                handle_alloc_error(layout);
            }
        } else {
            // Spin loop waiting on the mmap to be ready.
            while 0 == inner.offset.load(Ordering::Relaxed) {}
        }

        let bytes_required = align_to(layout.size() + layout.align(), layout.align());

        let my_offset = inner.offset.fetch_add(bytes_required, Ordering::Relaxed);

        let aligned_offset = align_to(my_offset, layout.align());

        if (aligned_offset + layout.size()) > self.size {
            handle_alloc_error(layout);
        }

        inner.mmap.offset(aligned_offset as isize)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}
