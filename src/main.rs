#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

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
    print::clear();
    logging::log("successfull boot!");
    logging::log("Hellö Wörld!");

    panic!("this is a terrible mistake!");

    loop {}
}
