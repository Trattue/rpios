#![no_std]
#![no_main]

#[cfg(not(test))]
use core::arch::asm;

mod boot;
mod bsp;
mod hal;
mod mmio;
mod print;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Kernel {}", _info);
    loop {
        unsafe { asm!("wfe") };
    }
}

unsafe fn main() {
    println!("Hello World!");
    panic!();
}
