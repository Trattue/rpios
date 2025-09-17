mod start {
    use core::arch::asm;

    #[unsafe(link_section = ".init")]
    #[unsafe(no_mangle)]
    pub extern "C" fn _start() -> ! {
        unsafe { asm!(include_str!("boot.s")) };
        loop {
            unsafe { asm!("wfe") };
        }
    }
}
