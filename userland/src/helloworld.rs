#![no_std]
#![no_main]

mod libc;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // TODO print something
    loop {}
}

#[no_mangle]
// Inside here the CPL register should be 3 (CPL=3) --> we are in user land / ring 3
pub extern "C" fn main() {
    loop {
        printf!("Hellö Wörld! I am process {}", libc::getpid());
    }
}
