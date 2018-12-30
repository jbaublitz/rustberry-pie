.section ".text.boot"

.globl _start
_start:
    mrs x0, mpidr_el1
    and x0, x0, #0xFF
    cbz x0, master
    b proc_down

master:
    adr x0, bss_begin
    mov x1, #0
    adr x2, bss_end
    sub x2, x2, x0
    bl memset

    mov x0, #4
    lsl x0, x0, #20
    mov sp, x0
    bl kernel_main

.section ".text"
proc_down:
    mov w0, #0x000B
    movk w0, #0x8400, lsl #16
    smc #0
    b system_down

system_down:
    mov w0, #0x0008
    movk w0, #0x8400, lsl #16
    smc #1
