fn main() {
    let bios_path = env!("BIOS_PATH");

    std::process::Command::new("qemu-system-x86_64")
        .arg("-drive")
        .arg(format!("format=raw,file={bios_path}"))
        .arg("-serial")
        .arg("stdio")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
