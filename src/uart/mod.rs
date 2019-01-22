use core::ptr;

pub struct GPIO<T> {
    bitfield_sel0: Option<u32>,
    bitfield_sel1: Option<u32>,
    bitfield_sel2: Option<u32>,
    bitfield_sel3: Option<u32>,
    bitfield_sel4: Option<u32>,
    proto: T,
}

pub struct UART;

impl<T> GPIO<T> {
    const R_GPFSEL0: *const u32 = 0x7e200004 as *const u32;
    const W_GPFSEL0: *mut u32 = 0x7e200004 as *mut u32;

    const R_GPFSEL1: *const u32 = 0x7e200004 as *const u32;
    const W_GPFSEL1: *mut u32 = 0x7e200004 as *mut u32;

    const R_GPFSEL2: *const u32 = 0x7e200004 as *const u32;
    const W_GPFSEL2: *mut u32 = 0x7e200004 as *mut u32;

    const R_GPFSEL3: *const u32 = 0x7e200004 as *const u32;
    const W_GPFSEL3: *mut u32 = 0x7e200004 as *mut u32;

    const R_GPFSEL4: *const u32 = 0x7e200004 as *const u32;
    const W_GPFSEL4: *mut u32 = 0x7e200004 as *mut u32;
}

impl GPIO<UART> {
    pub fn new() -> Self {
        let bitfield_sel1 = Some(unsafe { ptr::read(Self::R_GPFSEL1) });
        GPIO { bitfield_sel0: None,
               bitfield_sel1,
               bitfield_sel2: None,
               bitfield_sel3: None,
               bitfield_sel4: None,
               proto: UART,
        }
    }

    pub fn clean_pin(&mut self, offset: u8) {
        self.bitfield_sel1 = self.bitfield_sel1.map(|bf| bf & !(0x7 << offset * 3))
    }
}

impl<UART> Drop for GPIO<UART> {
    fn drop(&mut self) {
        self.bitfield_sel1.map(|bf| unsafe { ptr::write(Self::W_GPFSEL1, bf) });
    }
}

pub fn hello_world_uart() -> Result<(), ()> {
    let mut gpio_uart = GPIO::<UART>::new();
    gpio_uart.clean_pin(4);
    gpio_uart.clean_pin(5);
    Ok(())
}
