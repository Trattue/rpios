#![no_std]
#![no_main]

mod print;

// Retrieve _start function from BSPs
#[cfg(feature = "raspi3b")]
pub use bsp_raspi3b::boot;

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    println!("Kernel {}", _info);
    loop {
        unsafe {
            use core::arch::asm;
            asm!("wfe")
        };
    }
}

#[unsafe(no_mangle)]
fn main() {
    println!("Hello World!");
}
