#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::{arch::asm, panic::PanicInfo};

mod gdt;
mod interrupt;
mod keyboard;
mod logging;
mod print;
mod time;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logging::log("Kernel Panic!");

    println!("{}", info);

    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    gdt::init_gdt();
    interrupt::init_idt();

    clear_console!();
    println!("successfull boot!");
    println!("Hellö Wörld!");

    // Trigger exception
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    //panic!("this is a terrible mistake!");

    loop {}
}
