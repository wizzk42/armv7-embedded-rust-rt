//! bootstrap.rs
//! Provides the entry point bootstrap

use core::panic::PanicInfo;
use core::ptr;

/// main entry point contract
///
/// # Safety
///     bootstrap routine to initialize memory and run main
#[no_mangle]
pub unsafe extern "C" fn reset() -> ! {

    extern "C" {
        static mut _SBSS: u8;
        static mut _EBSS: u8;

        static mut _SDATA: u8;
        static mut _EDATA: u8;
        static _SIDATA: u8;
    }

    // initialize .bss to avoid UB
    // normally .bss contains uninitialized data
    let count = &_EBSS as *const u8 as usize - &_SBSS as *const u8 as usize;
    ptr::write_bytes(&mut _SBSS as *mut u8, 0, count);

    // initialize .data segment to avoid UB
    let count = &_EDATA as *const u8 as usize - &_SDATA as *const u8 as usize;
    ptr::copy_nonoverlapping(&_SIDATA as *const u8, &mut _SDATA as *mut u8, count);

    extern "Rust" {

        fn main() -> !;

        fn panic(_info: &PanicInfo<'_>) -> !;

        fn default_exception_handler() -> !;
    }

    main()
}

/// __RESET_VECTOR fn pointer
///     A static fn pointer to the reset bootstrap
///
/// # Safety
///     reset is unsafe so this is unsafe to.
///
#[link_section = ".vector_table.reset_vector"]
#[no_mangle]
pub static __RESET_VECTOR: unsafe extern "C" fn() -> ! = reset;

/// entry macro
///     A macro to set a function as the main entry point to
///     be called by reset
#[macro_export]
macro_rules! entry {
    ($path:path) => {
        #[export_name = "main"]
        pub unsafe fn __main() -> ! {
            let f: fn() -> ! = $path;
            f()
        }
    }
}
