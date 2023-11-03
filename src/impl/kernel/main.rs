#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod print;
mod time;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    print::clear();
    print::print_line("Hello World!");

    print::print_line("Und nu?\n");

    print::print_line("Test");

    time::get_time();

    loop {}
}
