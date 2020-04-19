//! exception.rs
//! Provides exception handling

pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

extern "C" {
    fn nmi();
    fn __hard_fault_trampoline();
    fn mem_manage();
    fn bus_fault();
    fn usage_fault();
    fn sv_call();
    fn pend_sv();
    fn sys_tick();
}

/// exception handler table
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { handler: nmi },
    Vector { handler: __hard_fault_trampoline },
    Vector { handler: mem_manage },
    Vector { handler: bus_fault },
    Vector { handler: usage_fault, },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: sv_call },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: pend_sv },
    Vector { handler: sys_tick },
];

/// default exception handler: loops indefinitely
#[no_mangle]
pub extern "C" fn default_exception_handler() {
    loop {}
}
