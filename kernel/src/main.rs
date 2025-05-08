#![no_std]
#![no_main]

use kernel::dbg_println;

use core::panic::PanicInfo;

fn kernel_main(_boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    dbg_println!("Hello, debugging!");

    loop {}
}

bootloader_api::entry_point!(kernel_main);

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    dbg_println!("{}", info);

    loop {}
}
