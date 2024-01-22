#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::{arch::asm, panic::PanicInfo};

mod gdt;
mod interrupt;
mod keyboard;
mod kprint;
mod libc;
mod logging;
mod process;
mod syscall;
mod time;
mod userland;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logging::log("Kernel Panic!");

    kprintln!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    gdt::init_gdt();
    interrupt::init_idt();

    clear_console!();
    kprintln!("successfull boot!");
    kprintln!("Hellö Wörld!");

    // Trigger exception
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    let userland: userland::Userland = userland::Userland::new();

    userland.switch_to_userland();

    //panic!("this is a terrible mistake!");

    loop {}
}
