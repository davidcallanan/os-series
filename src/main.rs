#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod interrupt;
mod logging;
mod print;
mod time;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    logging::log("Kernel Panic!");

    let msg = match info.payload().downcast_ref::<&'static str>() {
        Some(s) => *s,
        None => "  No further details",
    };

    logging::log(msg);

    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    interrupt::init_idt();

    clear_console!();
    print_line!("successfull boot!");
    print_line!("Hellö Wörld!");

    let mut counter = 0;

    loop {
        print_line!("Counter {}", counter);
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
