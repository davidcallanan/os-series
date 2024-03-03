use core::arch::asm;

extern "C" {
    fn trigger_syscall();
}

//pid_t getppid(void);
pub fn getpid() -> u64 {
    let mut pid = core::u64::MAX;

    unsafe {
        asm!("
            push rdx
            push rcx

            mov rdx, {0:r}

            push r11
            push rcx
        
            syscall
        
            pop rcx
            pop r11

            mov {1:r}, rdx

            pop rcx
            pop rdx
        ",
            in(reg) 2,
            out(reg) pid
        );
    }

    pid
}

pub fn write(filedescriptor: i64, payload: &[u8]) {
    unsafe {
        asm!("
            push rdx
            push r10
            push r8
            push r9
            push rcx

            mov rdx, 1
            mov r10, {0:r}
            mov r8, {1:r}
            mov r9, {2:r}

            push r11
            push rcx
        
            syscall
        
            pop rcx
            pop r11

            pop rcx
            pop r9
            pop r8
            pop r10
            pop rdx
        ",
            in(reg) filedescriptor,
            in(reg) payload.as_ptr(),
            in(reg) payload.len(),
            out("rcx") _,
            out("r9") _,
            out("r8") _,
            out("r10") _,
            out("rdx") _,
        );
    }
}

pub struct Printer {}

impl core::fmt::Write for Printer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        write(1, s.as_bytes());
        Ok(())
    }
}

#[macro_export]
macro_rules! printf {
    () => {    };
    ($($arg:tt)*) => {{
        let mut printer = crate::libc::Printer {};
        core::fmt::write(&mut printer, core::format_args!($($arg)*)).unwrap();
    }};
}
