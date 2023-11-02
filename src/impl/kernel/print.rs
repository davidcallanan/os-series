#[allow(dead_code)]
#[repr(u8)]
pub enum Colors {
    PrintColorBlack = 0,
    PrintColorBlue = 1,
    PrintColorGreen = 2,
    PrintColorCyan = 3,
    PrintColorRed = 4,
    PrintColorMagenta = 5,
    PrintColorBrown = 6,
    PrintColorLightGray = 7,
    PrintColorDarkGray = 8,
    PrintColorLightBlue = 9,
    PrintColorLightGreen = 10,
    PrintColorLightCyan = 11,
    PrintColorLightRed = 12,
    PrintColorPink = 13,
    PrintColorYellow = 14,
    PrintColorWhite = 15,
}

fn get_video_byte_string(character: char, foreground: Colors, background: Colors) -> u16 {
    let backgroundu8 = background as u16;
    let foregroundu8 = foreground as u16;
    return (backgroundu8 << 4 | foregroundu8) << 8 | character as u16;
}

// TODO handle those module global variables in rusty way
static mut current_row: u64 = 0;
static mut current_col: u64 = 0;

pub fn clear() {
    for column in 0..80 {
        for row in 0..25 {
            unsafe {
                // https://en.wikipedia.org/wiki/VGA_text_mode
                core::ptr::write_volatile(
                    (0xb8000 + (row * 80 + column) * 2) as *mut u16,
                    get_video_byte_string(' ', Colors::PrintColorBlack, Colors::PrintColorWhite),
                );
            }
        }
    }
}

pub fn print_line(text: &str) {
    for char in text.chars() {
        print_char(char);
    }
}

pub fn print_char(character: char) {
    unsafe {
        // https://en.wikipedia.org/wiki/VGA_text_mode
        core::ptr::write_volatile(
            (0xb8000 + (current_col + current_row * 80) * 2) as *mut u16,
            get_video_byte_string(character, Colors::PrintColorBlack, Colors::PrintColorWhite),
        );

        if current_col == 80 {
            current_col = 0;
            current_row += 1;
        }
        current_col += 1;
    }
}
