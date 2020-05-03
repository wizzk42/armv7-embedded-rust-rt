//! interrupts.rs
//! provides: interrupt routines

pub use bare_metal::{CriticalSection, Mutex};

#[doc(hidden)]
extern "C" {
    fn __irq_handler_trampoline();
}

#[doc(hidden)]
#[link_section = ".vector_table.interrupts"]
#[no_mangle]
pub static mut __INTERRUPTS: [unsafe extern "C" fn(); 240] = [{
    __irq_handler_trampoline
}; 240];

#[doc(hidden)]
#[inline]
pub fn disable() {
    extern "C" {
        fn __cpsid();
    }

    unsafe { __cpsid(); }
}

#[doc(hidden)]
#[inline]
pub unsafe fn enable() {
    extern "C" {
        fn __cpsie();
    }

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

/// default interrupt handler: empty
#[no_mangle]
pub extern "C" fn irq_handler(_irq: u8) {}
