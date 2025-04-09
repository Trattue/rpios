#![no_std]
#![no_main]

use core::arch::asm;

mod start {
    use core::arch::asm;

    #[unsafe(link_section = ".init")]
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() -> ! {
        unsafe { asm!(include_str!("start.s")) };
        loop {
            unsafe { asm!("wfe") };
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}

#[unsafe(no_mangle)]
pub unsafe fn _start_rust() -> ! {
    loop {
        unsafe { asm!("add x0, x0, #1") };
    }
}
