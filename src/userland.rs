use crate::gdt::TSS_ENTRY;
use crate::printf;
use core::arch::asm;

pub fn switch_to_userland() {
    extern "C" {
        fn jump_usermode();
    }

    unsafe {
        asm!("mov {}, rsp", out(reg) TSS_ENTRY.rsp0 );

        jump_usermode();
    }
}

#[no_mangle]
// Inside here the CPL register should be 3 (CPL=3) --> we are in user land / ring 3
pub extern "C" fn userland() {
    loop {
        printf!("foo");
        //kprintln!("Hellö Wörld!");
    }
}
