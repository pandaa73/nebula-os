#![no_std]
#![no_main]

mod debug;

use core::panic::PanicInfo;

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    println!("Hello, debugging!");

    loop {}
}

bootloader_api::entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}
