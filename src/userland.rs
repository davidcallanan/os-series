use crate::gdt::TSS_ENTRY;
use core::arch::asm;

pub fn switch_to_userland() {
    extern "C" {
        fn jump_usermode();
    }

    unsafe {
        // TODO set dynamically before switching to userspace
        TSS_ENTRY.rsp0 = 0x14cf68;
        jump_usermode();
    }
}

#[no_mangle]
// Inside here the CPL register should be 3 (CPL=3) --> we are in user land / ring 3
pub extern "C" fn userland() {
    // System call
    // TODO Renable
    // TODO wrap nicely ("glibc"?)
    /*unsafe {
        asm!(
            "
            push r11
            push rcx
            syscall
            pop rcx
            pop r11
        "
        );
    }*/

    loop {}
}

#[no_mangle]
// Inside here the CPL register should be 3 (CPL=3) --> we are in user land / ring 3
pub extern "C" fn userland_loop() {
    loop {}
}
