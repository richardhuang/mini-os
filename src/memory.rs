//! 基础内存管理模块

use core::alloc::{GlobalAlloc, Layout};
use core::ptr;
use spin::Mutex;

pub struct BumpAllocator {
    heap_start: usize,
    heap_end: usize,
    next: Mutex<usize>,
}

impl BumpAllocator {
    pub const fn new(heap_start: usize, heap_size: usize) -> Self {
        BumpAllocator {
            heap_start,
            heap_end: heap_start + heap_size,
            next: Mutex::new(heap_start),
        }
    }

    fn allocate(&self, layout: Layout) -> *mut u8 {
        let mut next = self.next.lock();
        let alloc_start = align_up(*next, layout.align());
        let alloc_end = alloc_start.saturating_add(layout.size());

        if alloc_start > self.heap_end || alloc_end > self.heap_end {
            return ptr::null_mut();
        }

        *next = alloc_end;
        alloc_start as *mut u8
    }
}

fn align_up(addr: usize, align: usize) -> usize {
    (addr + align - 1) & !(align - 1)
}

#[global_allocator]
static ALLOCATOR: BumpAllocator = BumpAllocator::new(0x100000000000, 0x100000);

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        self.allocate(layout)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        // Bump allocator does not support deallocation
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: Layout) -> ! {
    panic!("Allocation error: {:?}", layout);
}

pub fn init() {
    crate::println!("[OK] Memory management initialized");
}
