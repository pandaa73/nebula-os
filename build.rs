use std::path::PathBuf;
use std::env::var_os;

fn main() {
    let out_dir = PathBuf::from(
        var_os("OUT_DIR")
        .unwrap()
    );

    let kernel = PathBuf::from(
        var_os("CARGO_BIN_FILE_KERNEL_kernel")
        .unwrap()
    );

    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .unwrap();

    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
