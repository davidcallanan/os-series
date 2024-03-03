use crate::gdt::TSS_ENTRY;
use crate::process::{Process, CURRENT_PROCESS};
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
            fn jump_usermode(process_base_address: u64, stack_top_address: u64, entry_address: u64);
        }

        unsafe {
            let mut rsp0: u64;

            asm!("mov {}, rsp", out(reg) rsp0);

            TSS_ENTRY.rsp0 = rsp0;

            CURRENT_PROCESS = 0;

            jump_usermode(
                self.process.get_c3_page_map_l4_base_address(),
                self.process.get_stack_top_address(),
                self.process.get_entry_ip(),
            );
        }
    }
}
