#![no_std]  // no std library
#![no_main] // no main fn

use core::panic::PanicInfo;

/// Function is called on a panic
#[panic_handler]    // tells compiler that this is the panic handler; only one allowed per program
fn panic(_info: & PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for(i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop{}
}