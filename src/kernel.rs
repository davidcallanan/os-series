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

    let mut counter = 0;

    // Trigger exception
    unsafe {
        asm!("int3", options(nomem, nostack));
    }

    loop {
        println!("Counter {}", counter);
        logging::log(" ");

        // TODO implement a sleep function
        for _ in 0..10000000 {
            ()
        }

        counter += 1;
    }

    //panic!("this is a terrible mistake!");

    //loop {}
}
