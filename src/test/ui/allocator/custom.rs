// run-pass

// aux-build:helper.rs
// no-prefer-dynamic

#![feature(allocator_api)]
#![feature(slice_ptr_get)]

extern crate helper;

use std::alloc::{self, AllocRef, Global, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::ptr::NonNull;

static HITS: AtomicUsize = AtomicUsize::new(0);

struct A;

unsafe impl alloc::GlobalAlloc for A {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        HITS.fetch_add(1, Ordering::SeqCst);
        AllocRef::alloc(&System, layout).unwrap().as_mut_ptr()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        HITS.fetch_add(1, Ordering::SeqCst);
        AllocRef::dealloc(&System, NonNull::new(ptr).unwrap(), layout)
    }
}

#[global_allocator]
static GLOBAL: A = A;

fn main() {
    println!("hello!");

    let n = HITS.load(Ordering::SeqCst);
    assert!(n > 0);
    unsafe {
        let layout = Layout::from_size_align(4, 2).unwrap();

        let memory = Global.alloc(layout.clone()).unwrap();
        helper::work_with(&memory);
        assert_eq!(HITS.load(Ordering::SeqCst), n + 1);
        Global.dealloc(memory.as_non_null_ptr(), layout);
        assert_eq!(HITS.load(Ordering::SeqCst), n + 2);

        let s = String::with_capacity(10);
        helper::work_with(&s);
        assert_eq!(HITS.load(Ordering::SeqCst), n + 3);
        drop(s);
        assert_eq!(HITS.load(Ordering::SeqCst), n + 4);

        let memory = System.alloc(layout.clone()).unwrap();
        assert_eq!(HITS.load(Ordering::SeqCst), n + 4);
        helper::work_with(&memory);
        System.dealloc(memory.as_non_null_ptr(), layout);
        assert_eq!(HITS.load(Ordering::SeqCst), n + 4);
    }
}
