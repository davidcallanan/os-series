use crate::USERLAND;
use crate::{kprintln, logging::log};
use core::arch::asm;

#[no_mangle]
pub extern "C" fn system_call() -> u64 {
    let mut syscall_nr: i64;

    unsafe {
        asm!("
        ",
            out("rdi") syscall_nr
        );
    }

    match syscall_nr {
        1 => return syscall_write(),
        2 => return syscall_getpid(),
        _ => {
            kprintln!("Undefined system call triggered");
            return 0xdeadbeef;
        }
    }
}

fn syscall_getpid() -> u64 {
    USERLAND.lock().get_current_process_id() as u64
}

fn syscall_write() -> u64 {
    let mut filedescriptor: i64;
    let mut payload: i64;
    let mut len: i64;

    unsafe {
        // TODO this must be possible more elegantly
        asm!("",
            out("r8") filedescriptor,
            out("r9") payload,
            out("r10") len
        );

        match core::str::from_utf8(core::slice::from_raw_parts(
            payload as *const u8,
            len as usize,
        )) {
            Ok(msg) => match filedescriptor {
                // stdout
                1 => {
                    kprintln!("{}", msg)
                }
                _ => log("Undefined filedescriptor!"),
            },
            Err(_) => kprintln!("\nCouldnt reconstruct string!\n"),
        }
    }

    return 0;
}
