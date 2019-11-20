#![unstable(feature = "thread_local_internals", issue = "0")]
#![cfg(target_thread_local)]

use crate::cell::Cell;
use crate::sys::c;

type List = Vec<(*mut u8, unsafe extern "C" fn(*mut u8))>;

#[thread_local]
static DTORS: Cell<List> = Cell::new(List::new());

pub(crate) unsafe fn register_dtor(t: *mut u8, dtor: unsafe extern "C" fn(*mut u8)) {
    let mut dtors = DTORS.take();
    dtors.push((t, dtor));
    let old_dtors = DTORS.replace(dtors);
    rtassert!(old_dtors.is_empty()); // TODO: is this really needed?
}

unsafe fn run_dtors() {
    let mut dtors = DTORS.take();
    while !dtors.is_empty() {
        dtors.iter().for_each(|&(ptr, dtor)| dtor(ptr)); // TODO: does the order matter?
        dtors = DTORS.take();
    }
}

#[link_section = ".CRT$XLB"]
#[used] // we don't want LLVM eliminating this symbol for any reason, and
        // when the symbol makes it to the linker the linker will take over
static p_thread_callback: unsafe extern "system" fn(c::LPVOID, c::DWORD,
                                                        c::LPVOID) =
        on_tls_callback;

unsafe extern "system" fn on_tls_callback(_h: c::LPVOID,
                                          dwReason: c::DWORD,
                                          _pv: c::LPVOID) {
    if dwReason == c::DLL_THREAD_DETACH || dwReason == c::DLL_PROCESS_DETACH {
        run_dtors();
    }
}
