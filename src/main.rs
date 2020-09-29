#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

extern {
    static __bss_start: usize;
    static __bss_end: usize;
}

#[no_mangle]
pub extern "C" fn _wumbo() -> ! {
    panic!()
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        loop {
            asm!("wfe")
        }
    }
}
