use crate::print::{print_char, print_char_at_pos, print_integer, print_integer_at_pos};
use core::arch::asm;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
enum CmosRegister {
    Seconds = 0x00,
    Minutes = 0x02,
    Hours = 0x04,
    Weekday = 0x06,
    DayOfMonth = 0x07,
    Month = 0x08,
    Year = 0x09,
    StatusA = 0x0a,
    StatusB = 0x0b,
}

// https://wiki.osdev.org/CMOS#Accessing_CMOS_Registers
fn read_cmos_i16(register: CmosRegister, bcd_enabled: bool) -> i16 {
    unsafe {
        asm!(
            r#"out %al, %dx"#,
            in("dx") 0x70 as i16,
            in("al") register.clone() as u8,
            options(att_syntax)
        );

        let mut ret: i8;

        asm!(
            r#"in %dx, %al"#,
            in("dx") 0x71 as i16,
            out("al") ret,
            options(att_syntax)
        );

        if bcd_enabled && register != CmosRegister::StatusA {
            return ((ret as i16 & 0xF0) >> 1) + ((ret as i16 & 0xF0) >> 3) + (ret as i16 & 0xf);
        } else {
            return ret as i16;
        }
    }
}

// https://github.com/sphaerophoria/stream-os/blob/master/src/io/io_allocator.rs#L67
// https://stackoverflow.com/a/64818139
pub fn print_time() {
    let bcd_enabled: bool = read_cmos_i16(CmosRegister::StatusA, false) != 0;

    let hours: i16 = read_cmos_i16(CmosRegister::Hours, bcd_enabled);
    let minutes: i16 = read_cmos_i16(CmosRegister::Minutes, bcd_enabled);
    let seconds: i16 = read_cmos_i16(CmosRegister::Seconds, bcd_enabled);

    print_integer(hours.into());
    print_char(':');
    print_integer(minutes.into());
    print_char(':');
    print_integer(seconds.into());
}

pub fn update_clock() {
    let bcd_enabled: bool = read_cmos_i16(CmosRegister::StatusA, false) != 0;

    let hours: i16 = read_cmos_i16(CmosRegister::Hours, bcd_enabled);
    let minutes: i16 = read_cmos_i16(CmosRegister::Minutes, bcd_enabled);
    let seconds: i16 = read_cmos_i16(CmosRegister::Seconds, bcd_enabled);

    print_integer_at_pos(hours.into(), 0, 70);
    print_char_at_pos(':', 0, 72);
    print_integer_at_pos(minutes.into(), 0, 73);
    print_char_at_pos(':', 0, 75);
    print_integer_at_pos(seconds.into(), 0, 76);
}
