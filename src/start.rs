#![no_std]
#![no_main]

mod start {
    use core::arch::asm;

    #[unsafe(link_section = ".init")]
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() -> ! {
        loop {
            unsafe { asm!("wfe") };
        }
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unimplemented!()
}
