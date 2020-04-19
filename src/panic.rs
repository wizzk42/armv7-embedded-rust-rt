//! panic.rs
//! provides: a default panic handler function

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info : &PanicInfo<'_>) -> ! {
    loop {}
}
