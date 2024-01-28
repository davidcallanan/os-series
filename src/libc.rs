use core::arch::asm;

//pid_t getppid(void);
pub fn getpid() -> u64 {
    let mut pid = core::u64::MAX;
    unsafe {
        asm!("
            push rax

            mov rax, {0:r}

            call trigger_syscall

            mov {1:r}, rax

            pop rax
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
            push rax
            push rbx
            push r8
            push rdx

            mov rax, {0:r}
            mov rbx, {1:r}
            mov r8, {2:r}
            mov rdx, {3:r}

            call trigger_syscall

            pop rdx
            pop r8
            pop rbx
            pop rax
        ",
            in(reg) 1,
            in(reg) filedescriptor,
            in(reg) payload.as_ptr(),
            in(reg) payload.len()
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
