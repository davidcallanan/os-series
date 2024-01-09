// add better formatting options, see https://os.phil-opp.com/vga-text-mode/#a-println-macro

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
static mut CURRENT_ROW: u64 = 0;
static mut CURRENT_COL: u64 = 0;

pub struct Printer {}

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        crate::print::print(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! println {
    () => {
        crate::print::print_char('\n');
    };
    ($($arg:tt)*) => {{
        let mut printer = crate::print::Printer {};
        core::fmt::write(&mut printer, core::format_args!($($arg)*)).unwrap();
        crate::print::print_char('\n');
    }};
}

#[macro_export]
macro_rules! print {
    () => {};
    ($($arg:tt)*) => {{
        let mut printer = crate::print::Printer {};
        core::fmt::write(&mut printer, core::format_args!($($arg)*)).unwrap();
    }};
}

#[macro_export]
macro_rules! clear_console {
    () => {
        crate::print::clear();
    };
}

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

fn scroll_line() {
    for column in 0..80 {
        for row in 0..24 {
            // Exception for clock
            if row == 0 && column >= 70 {
                continue;
            }
            unsafe {
                core::ptr::write_volatile(
                    (0xb8000 + (row * 80 + column) * 2) as *mut u16,
                    core::ptr::read_volatile((0xb8000 + ((row + 1) * 80 + column) * 2) as *mut u16),
                );
            }
        }
    }

    // clear bottom line
    for column in 0..80 {
        unsafe {
            core::ptr::write_volatile(
                (0xb8000 + (24 * 80 + column) * 2) as *mut u16,
                get_video_byte_string(' ', Colors::PrintColorBlack, Colors::PrintColorWhite),
            );
        }
    }

    unsafe {
        CURRENT_COL = 0;
        CURRENT_ROW = 0;
    }
}

pub fn print(text: &str) {
    for character in text.chars() {
        print_char(character);
    }
}

pub fn print_line(text: &str) {
    print(text);
    print_char('\n');
}

pub fn print_text_at_pos(text: &str, row: u64, column: u64) {
    let mut i = 0;
    for character in text.chars() {
        print_char_at_pos(character, row, column + i);
        i += 1;
    }
}

pub fn print_char_at_pos(character_in: char, row: u64, column: u64) {
    // TODO remove the unsafe
    unsafe {
        let old_row = CURRENT_ROW;
        let old_column = CURRENT_COL;

        CURRENT_ROW = row;
        CURRENT_COL = column;

        print_char(character_in);

        CURRENT_COL = old_column;
        CURRENT_ROW = old_row;
    }
}

pub fn print_char(character_in: char) {
    let mut character = character_in;

    match character as u8 {
        0x20..=0x7e | b'\n' => (),
        _ => character = 0xfe as char,
    }

    unsafe {
        if character == '\n' {
            CURRENT_ROW += 1;

            if CURRENT_ROW > 24 {
                scroll_line();
                CURRENT_ROW = 24;
            }

            CURRENT_COL = 0;
            return;
        }

        // https://en.wikipedia.org/wiki/VGA_text_mode
        core::ptr::write_volatile(
            (0xb8000 + (CURRENT_COL + CURRENT_ROW * 80) * 2) as *mut u16,
            get_video_byte_string(character, Colors::PrintColorBlack, Colors::PrintColorWhite),
        );

        if CURRENT_COL == 80 {
            CURRENT_COL = 0;
            CURRENT_ROW += 1;

            if CURRENT_ROW > 24 {
                scroll_line();
                CURRENT_ROW = 24;
            }
        }
        CURRENT_COL += 1;
    }
}

pub fn print_integer(number: i64) {
    if number > 10 {
        print_integer(number / 10);
    }
    print_char((number % 10 + 0x30) as u8 as char);
}

pub fn print_integer_at_pos(number: i64, row: u64, column: u64) {
    // TODO remove the unsafe
    unsafe {
        let old_row = CURRENT_ROW;
        let old_column = CURRENT_COL;

        CURRENT_ROW = row;
        CURRENT_COL = column;

        if number > 10 {
            print_integer_at_pos(number / 10, row, column);
            CURRENT_COL = column + 1;
        }
        print_char((number % 10 + 0x30) as u8 as char);

        CURRENT_COL = old_column;
        CURRENT_ROW = old_row;
    }
}
