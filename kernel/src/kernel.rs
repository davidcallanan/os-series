#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use lazy_static::lazy_static;
use spin::Mutex;

mod gdt;
mod interrupt;
mod keyboard;
mod kprint;
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

lazy_static! {
    pub static ref USERLAND: Mutex<userland::Userland> = Mutex::new(userland::Userland::new());
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    gdt::init_gdt();
    interrupt::init_idt();

    clear_console!();
    kprintln!("successfull boot!");
    kprintln!("Hellö Wörld!");

    // Trigger test exception
    //unsafe {
    //    asm!("int3", options(nomem, nostack));
    //}

    USERLAND.lock().switch_to_userland(&USERLAND);

    panic!("This should never happen!?");
}
