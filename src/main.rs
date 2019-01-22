#![no_std]
#![no_main]
#![feature(align_offset,asm)]

use core::panic::PanicInfo;

mod asm;
mod intr;
mod uart;

extern "C" {
    fn system_down() -> !;
}

#[panic_handler]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { system_down() }
}

#[link_section = ".text"]
#[no_mangle]
pub fn kernel_main() {
    match uart::hello_world_uart() {
        Ok(()) => (),
        Err(()) => {
            unsafe { system_down() };
        }
    };
}
