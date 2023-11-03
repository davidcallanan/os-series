use core::arch::asm;
use print::{print_integer, print_line};

// https://wiki.osdev.org/CMOS#Accessing_CMOS_Registers
// https://github.com/sphaerophoria/stream-os/blob/master/src/io/io_allocator.rs#L67
// https://stackoverflow.com/a/64818139
// outb (0x70, (NMI_disable_bit << 7) | (selected CMOS register number));
pub fn get_time() {
    unsafe {
        asm!(
            r#"out %al, %dx"#,
            in("dx") 0x70 as i16,
            in("al") 0x08 as u8,
            options(att_syntax)
        );

        let mut ret: i8;

        asm!(
            r#"in %dx, %al"#,
            in("dx") 0x71 as i16,
            out("al") ret,
            options(att_syntax)
        );

        print_line("Time: ");
        print_integer(ret.into());
        print_line("seconds");
    }
}
