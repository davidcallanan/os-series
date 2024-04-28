#![no_std]
#![no_main]

mod libc;
use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // TODO print something
    loop {}
}

#[no_mangle]
pub fn _start() {
    loop {
        libc::getpid();
        printf!("Test\n");
        printf!("Hellö Wörld! I am process {}", libc::getpid());
    }
}
