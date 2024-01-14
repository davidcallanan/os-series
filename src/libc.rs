use core::arch::asm;

//ssize_t write(int fd, const void buf[.count], size_t count);

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

            jmp trigger_syscall

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
