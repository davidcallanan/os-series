#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

struct Char {
    character: u8,
    color: u8,
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let NUM_COLS: usize = 80;
    let NUM_ROWS: usize = 25;

    let buffer = 0xb8000 as * mut Char;

    let col: usize = 0;
    let row: usize = 0;
    //let color: u8  = PRINT_COLOR_WHITE | PRINT_COLOR_BLACK << 4;
    // see print.h
    let color: u8  = 15 | 0 << 4;

    let x = Char {
        character: 'X' as u8,
        color: color,
    };

    //buffer[col + NUM_COLS * row]= x;

    unsafe {
        core::ptr::write_volatile(0xb8000 as *mut u8, 0x58/* << 9 | 15 | 0 << 4*/);
    }

    loop {}
}