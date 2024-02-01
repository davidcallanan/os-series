use crate::process::CURRENT_PROCESS;
use crate::{kprintln, logging::log};
use core::arch::asm;

#[no_mangle]
pub extern "C" fn system_call() {
    let mut syscall_nr: i64;

    unsafe {
        asm!("
            mov {:r}, rax
        ",
            out(reg) syscall_nr
        );
    }

    match syscall_nr {
        1 => syscall_write(),
        2 => syscall_getpid(),
        _ => {
            kprintln!("System Call triggered");
        }
    }
}

fn syscall_getpid() {
    unsafe {
        asm!("
            mov rax, {:r}
        ",
            in(reg) CURRENT_PROCESS,
        );
    }
}

fn syscall_write() {
    let mut filedescriptor: i64;
    let mut payload: i64;
    let mut len: i64;
    let bytes: &str;

    unsafe {
        asm!("
            mov {:r}, rbx
        ",
            out(reg) filedescriptor,
        );
        asm!("
            mov {:r}, r8
        ",
            out(reg) payload,
        );
        asm!("
            mov {:r}, rdx
        ",
            out(reg) len
        );

        bytes = core::str::from_utf8(core::slice::from_raw_parts(
            payload as *const u8,
            len as usize,
        ))
        .unwrap();
    }

    match filedescriptor {
        // stdout
        1 => {
            kprintln!("{}", bytes)
        }
        _ => log("Undefined filedescriptor!"),
    }
}
