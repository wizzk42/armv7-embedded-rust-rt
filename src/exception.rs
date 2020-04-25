//! exception.rs
//! Provides exception handling

use core::fmt;

#[doc(hidden)]
#[allow(non_camel_case_types)]
pub enum ExceptionType {
    nmi,
    // Not overridable
    // HardFault,
    mem_manage,
    bus_fault,
    usage_fault,
    sv_call,
    debug_monitor,
    pend_sv,
    sys_tick,
}

#[doc(hidden)]
pub union Vector {
    reserved: u32,
    handler: unsafe extern "C" fn(),
}

#[doc(hidden)]
extern "C" {
    fn nmi();
    fn __hard_fault_trampoline();
    fn mem_manage();
    fn bus_fault();
    fn usage_fault();
    fn sv_call();
    fn debug_monitor();
    fn pend_sv();
    fn sys_tick();
}

#[doc(hidden)]
#[link_section = ".vector_table.exceptions"]
#[no_mangle]
pub static __EXCEPTIONS: [Vector; 14] = [
    Vector { handler: nmi },
    Vector { handler: __hard_fault_trampoline },
    Vector { handler: mem_manage },
    Vector { handler: bus_fault },
    Vector { handler: usage_fault },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { reserved: 0 },
    Vector { handler: sv_call },
    Vector { handler: debug_monitor },
    Vector { reserved: 0 },
    Vector { handler: pend_sv },
    Vector { handler: sys_tick },
];


/// Registers stacked (pushed into the stack) during an exception
#[derive(Clone, Copy)]
#[repr(C)]
pub struct ExceptionFrame {
    /// general purpose registers
    pub r0: u32,
    pub r1: u32,
    pub r2: u32,
    pub r3: u32,
    pub r12: u32,

    /// Linker Register
    pub lr: u32,

    /// Program Counter
    pub pc: u32,

    /// Program Status Register
    pub xpsr: u32,
}

impl fmt::Debug for ExceptionFrame {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        struct Hex(u32);
        impl fmt::Debug for Hex {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write!(f, "0x{:08x}", self.0)
            }
        }
        f.debug_struct("ExceptionFrame")
            .field("r0", &Hex(self.r0))
            .field("r1", &Hex(self.r1))
            .field("r2", &Hex(self.r2))
            .field("r3", &Hex(self.r3))
            .field("r12", &Hex(self.r12))
            .field("lr", &Hex(self.lr))
            .field("pc", &Hex(self.pc))
            .field("xpsr", &Hex(self.xpsr))
            .finish()
    }
}

#[macro_export]
macro_rules! exception {
    (*, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn default_exception_handler() {
            extern crate core;

            let f: fn(i16) = $handler;

            const SCB_ICSR: *const u32 = 0xE000_ED04 as *const u32; // SystemControlBlock of Interrupt Control & State Register mmapped addr

            f(core::ptr::read(SCB_ICSR) as u8 as i16 - 16)
        }
    };

    (hard_fault, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)] // raise an error if this item is not accessible
        #[no_mangle]
        pub unsafe extern "C" fn hard_fault(ef: &$crate::ExceptionFrame) {

            let f: fn(&$crate::ExceptionFrame) -> ! = $handler;

            f(ef)
        }
    };

    ($Name:ident, $handler:path, state: $State:ty = $initial_state:expr) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {
            static mut STATE: $State = $initial_state;

            let _ = $crate::ExceptionType::$Name;

            let f: fn(&mut $State) = $handler;

            f(&mut STATE)
        }
    };

    ($Name:ident, $handler:path) => {
        #[allow(unsafe_code)]
        #[deny(private_no_mangle_fns)]
        #[no_mangle]
        pub unsafe extern "C" fn $Name() {

            let _ = $crate::ExceptionType::$Name;

            let f: fn() = $handler;

            f()
        }
    };
}

/// default exception handler: loops indefinitely
#[no_mangle]
pub extern "C" fn default_exception_handler() {
    loop {}
}
