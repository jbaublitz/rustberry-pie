extern "C" {
    fn system_down() -> !;
}

#[link_section = ".text"]
#[no_mangle]
pub fn proc_down() -> ! {
    unsafe {
        asm!(
            "mov w0, #0x000B;
            movk w0, #0x8400, lsl #16;
            smc #0"
            : 
            :
            : "w0"
        );
    }
    unsafe { system_down() }
}
