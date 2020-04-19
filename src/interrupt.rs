//! interrupts.rs
//! provides: interrupt routines

pub use bare_metal::{CriticalSection, Mutex, Nr};

/// Disables all interrupts
#[inline]
pub fn disable() {
    extern "C" {
        fn __cpsid();
    }

    // XXX do we need a explicit compiler barrier here?
    unsafe { __cpsid(); }
}

#[inline]
pub unsafe fn enable() {
    extern "C" {
        fn __cpsie();
    }

    // XXX do we need a explicit compiler barrier here?
    __cpsie();
}

#[inline]
pub fn free<F, R>(f: F) -> R
    where
        F: FnOnce(&CriticalSection) -> R,
{
    let primask = crate::register::primask::read();

    disable();

    let r = f(unsafe { &CriticalSection::new() });

    if primask.is_active() {
        unsafe { enable() }
    }

    r
}
