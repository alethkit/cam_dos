#![no_std]
#![no_main]
#![feature(asm)]

use core::panic::PanicInfo;

mod gpio;
mod uart;
mod console;

extern "C" {
    static __bss_start: usize;
    static __bss_end: usize;
}

#[no_mangle]
pub extern "C" fn _wumbo() -> ! {
    unsafe {
        asm!("ldr {1}, {0}", "mov sp, {1}", sym _wumbo, out(reg) _);
        zero_memory_section(__bss_start as *mut usize, __bss_end as *mut usize);
    }
    let con = console::Console::new(0x3F21_5000, 0x3F20_0000);
    loop {
        con.send_char(con.recv_char());
    }
}

unsafe fn zero_memory_section(mut start_addr: *mut usize, end_addr: *mut usize) {
    while start_addr < end_addr {
        start_addr.write_volatile(0);
        start_addr = start_addr.offset(1);
    }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    unsafe {
        loop {
            asm!("wfe")
        }
    }
}
