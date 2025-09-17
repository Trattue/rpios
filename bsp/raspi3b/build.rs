use std::{env, path::PathBuf};

fn main() {
    // Pass linker script to kernel crate
    let linker_script = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
        .join("linker.ld")
        .canonicalize()
        .unwrap();
    println!(
        "cargo::metadata=KERNEL_LD_SCRIPT={}",
        linker_script.display()
    );

    // Kernel image name will be used in kernel crate
    println!("cargo::metadata=KERNEL_IMG_NAME=kernel8.img");
}
