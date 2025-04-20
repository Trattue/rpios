use crate::main;

mod start {
    use core::arch::asm;

    #[unsafe(link_section = ".init")]
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() -> ! {
        unsafe { asm!(include_str!("raspberry_pi_3b.s")) };
        loop {
            unsafe { asm!("wfe") };
        }
    }
}

#[unsafe(no_mangle)]
pub unsafe fn _start_rust() {
    unsafe { main() };
}
