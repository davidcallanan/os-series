#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[repr(packed)]
struct Char {
    character: u8,
    color: u8,
}

#[repr(u8)]
pub enum Colors {
    PrintColorBlack = 0,
    /*PRINT_COLOR_BLUE = 1,
    PRINT_COLOR_GREEN = 2,
    PRINT_COLOR_CYAN = 3,
    PRINT_COLOR_RED = 4,
    PRINT_COLOR_MAGENTA = 5,
    PRINT_COLOR_BROWN = 6,
    PRINT_COLOR_LIGHT_GRAY = 7,
    PRINT_COLOR_DARK_GRAY = 8,
    PRINT_COLOR_LIGHT_BLUE = 9,
    PRINT_COLOR_LIGHT_GREEN = 10,
    PRINT_COLOR_LIGHT_CYAN = 11,
    PRINT_COLOR_LIGHT_RED = 12,
    PRINT_COLOR_PINK = 13,
    PRINT_COLOR_YELLOW = 14,*/
    PrintColorWhite = 15,
}

fn get_video_byte_string(character: char, foreground: Colors, background: Colors) -> u16 {
    let backgroundu8 = background as u16;
    let foregroundu8 = foreground as u16;
    return (backgroundu8 << 4 | foregroundu8) << 8 | character as u16;
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    //let NUM_COLS: usize = 80;
    //let NUM_ROWS: usize = 25;

    let vga_buffer = 0xb8000 as *mut u16;

    //let col: usize = 0;
    //let row: usize = 0;

    //buffer[col + NUM_COLS * row]= x;

    unsafe {
        // https://en.wikipedia.org/wiki/VGA_text_mode

        // Works
        //core::ptr::write_volatile(0xb8000 as *mut u8, 0x58/* << 9 | 15 | 0 << 4*/);
        //core::ptr::write_volatile(0xb8001 as *mut u8, 0xb/* << 9 | 15 | 0 << 4*/);

        // | (Colors::PRINT_COLOR_BLACK as u8) << 4;

        core::ptr::write_volatile(
            vga_buffer,
            get_video_byte_string('Y', Colors::PrintColorBlack, Colors::PrintColorWhite),
        );
    }

    loop {}
}
