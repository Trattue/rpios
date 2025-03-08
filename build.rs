fn main() {
    register_link_script();
}

fn register_link_script() {
    println!("cargo:rerun-if-changed=raspi3b.ld");
}
