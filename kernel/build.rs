use std::env;

fn main() {
    // Use custom linker script from BSP
    let linker_script = env::var("DEP_BSP_KERNEL_LD_SCRIPT").expect("ld script from bsp crate");

    println!("cargo::rustc-link-arg=-T{}", linker_script);
    println!("cargo:rerun-if-changed={}", linker_script);
}
