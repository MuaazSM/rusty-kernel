#![no_std]  // no std library

use core::panic::PanicInfo;

/// Function is called on a panic
#[panic_handler]    // tells compiler that this is the panic handler; only one allowed per program
fn panic(_info: & PanicInfo) -> ! {
    loop {}
}

fn main() {}