use core::arch::asm;

pub fn out_port_b(port: u32, value: u8) {
    unsafe {
        asm!(
            r#"out %al, %dx"#,
            in("edx") port,
            in("al") value,
            options(att_syntax)
        );
    }
}

pub fn in_port_b(port: u32) -> u8 {
    let mut key;
    unsafe {
        asm!("in al, dx", out("al") key, in("rdx") port);
    }
    return key;
}
