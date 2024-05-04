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
    let mut x: u32 = 0;
    let mut y: u32 = 0;

    loop {
        libc::getpid();
        printf!("Test\n");
        printf!("Hellö Wörld! I am process {}", libc::getpid());
        libc::draw_pixel(x, y, libc::getpid() as u8);

        x += 1;

        if x == 320 {
            y += 1;
            x = 0;
        }

        if y == 200 {
            y = 0;
        }
    }
}
