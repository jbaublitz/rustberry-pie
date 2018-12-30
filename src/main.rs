#![no_std]
#![no_main]
#![feature(align_offset)]

use core::panic::PanicInfo;
use core::ptr;

extern "C" {
    fn system_down() -> !;
}

#[panic_handler]
#[link_section = ".text"]
pub fn panic_handler(_info: &PanicInfo) -> ! {
    unsafe { system_down() }
}

#[link_section = ".text"]
#[no_mangle]
pub fn kernel_main() {
}

#[link_section = ".text"]
#[no_mangle]
pub fn memset(addr: *mut u8, val: u8, len: u64) {
    for i in 0..len {
        unsafe { ptr::write_volatile(addr.offset(i as isize), val) }
    }
}
