use crate::gdt::TSS_ENTRY;
use crate::printf;
use crate::process::Process;
use core::arch::asm;

pub struct Userland {
    process: Process,
}

impl Userland {
    pub fn new() -> Self {
        Self {
            process: Process::new(),
        }
    }

    pub fn switch_to_userland(self) {
        extern "C" {
            fn jump_usermode(process_base_address: u64, stack_top_address: u64);
        }

        unsafe {
            asm!("mov {}, rsp", out(reg) TSS_ENTRY.rsp0);

            let process_base_address = self.process.getC3PageMapL4BaseAddress();

            let stack_top_address = self.process.getStackTopAddress();

            jump_usermode(process_base_address, stack_top_address);
        }
    }
}

#[no_mangle]
// Inside here the CPL register should be 3 (CPL=3) --> we are in user land / ring 3
pub extern "C" fn userland() {
    loop {
        printf!("Hellö Wörld!");
    }
}
