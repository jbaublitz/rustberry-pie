use core::ptr;

#[link_section = ".text"]
#[no_mangle]
pub fn memset(addr: *mut u8, val: u8, len: u64) {
    for i in 0..len {
        unsafe { ptr::write_volatile(addr.offset(i as isize), val) }
    }
}
