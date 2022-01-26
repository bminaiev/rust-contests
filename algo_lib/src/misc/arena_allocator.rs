use crate::collections::last_exn::LastExn;
use crate::dbg;
use std::alloc::{GlobalAlloc, Layout, System};
use std::cell::UnsafeCell;

pub struct SingleSizeFixedNumElementsAllocator {
    data: *mut u8,
    used_from: *mut u8,
    elem_size: usize,
}

const ALIGN: usize = 8;

impl SingleSizeFixedNumElementsAllocator {
    pub fn new(elem_size: usize, max_elems_num: usize) -> Self {
        let max_size = elem_size * max_elems_num;
        unsafe {
            let data = System.alloc(Layout::from_size_align(max_size, ALIGN).unwrap());
            let used_from = data.offset(max_size as isize);
            return Self {
                data,
                used_from,
                elem_size,
            };
        };
    }

    pub fn alloc(&mut self) -> *mut u8 {
        assert!(self.has_space());
        unsafe {
            self.used_from = self.used_from.sub(self.elem_size);
        }
        self.used_from
    }

    pub fn has_space(&self) -> bool {
        self.used_from != self.data
    }
}

pub struct SingleSizeAllocator {
    elem_size: usize,
    // Will this vector recursively use this allocator?
    parts: Vec<SingleSizeFixedNumElementsAllocator>,
    free_pointers: Vec<*mut u8>,
    total_allocations: u64,
}

impl SingleSizeAllocator {
    pub const fn new(elem_size: usize) -> Self {
        Self {
            elem_size,
            parts: Vec::new(),
            free_pointers: Vec::new(),
            total_allocations: 0,
        }
    }

    pub fn alloc(&mut self) -> *mut u8 {
        self.total_allocations += 1;
        if self.total_allocations & ((1 << 20) - 1) == 0 {
            dbg!(self.total_allocations);
        }
        if let Some(ptr) = self.free_pointers.pop() {
            return ptr;
        }
        if self.parts.is_empty() || !self.parts.last_exn().has_space() {
            let max_elems_num = 1usize << self.parts.len();
            self.parts.push(SingleSizeFixedNumElementsAllocator::new(
                self.elem_size,
                max_elems_num,
            ));
        }
        let last = self.parts.len() - 1;
        self.parts[last].alloc()
    }

    fn dealloc(&mut self, ptr: *mut u8, layout: Layout) {
        self.free_pointers.push(ptr);
    }
}

struct Inner {
    per_size: SingleSizeAllocator,
}

///
/// To use it just add:
/// ``
/// #[global_allocator]
/// static A : ArenaAlloc = ArenaAlloc::new();
/// ``
///
pub struct ArenaAlloc {
    inner: UnsafeCell<Inner>,
}

unsafe impl Sync for ArenaAlloc {}

const MAGIC_SIZE: usize = 48;

impl ArenaAlloc {
    pub const fn new() -> ArenaAlloc {
        ArenaAlloc {
            inner: UnsafeCell::new(Inner {
                per_size: SingleSizeAllocator::new(MAGIC_SIZE),
            }),
        }
    }
}

unsafe impl GlobalAlloc for ArenaAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        if layout.size() == MAGIC_SIZE {
            let inner = &mut *self.inner.get();
            inner.per_size.alloc()
        } else {
            System.alloc(layout)
        }
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        if layout.size() == MAGIC_SIZE {
            let inner = &mut *self.inner.get();
            inner.per_size.dealloc(ptr, layout)
        } else {
            System.dealloc(ptr, layout)
        }
    }
}
