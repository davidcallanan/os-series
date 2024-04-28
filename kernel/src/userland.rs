use heapless::Vec;
use spin::Mutex;

use crate::process::Process;
use crate::USERLAND;
use core::arch::global_asm;

global_asm!(include_str!("switch_to_ring3.S"));

//#[derive(Default)]
pub struct Userland {
    processes: Vec<Process, 4>,
    current_process: usize,
}

impl Userland {
    pub fn new() -> Self {
        let mut processes = Vec::<_, 4>::new();
        let _ = processes.push(Process::new());
        processes[0].initialize();
        let _ = processes.push(Process::new());
        processes[1].initialize();
        let _ = processes.push(Process::new());
        processes[2].initialize();
        let _ = processes.push(Process::new());
        processes[3].initialize();

        Self {
            processes: processes,
            current_process: 0,
        }
    }

    pub fn switch_to_userland(&mut self, mutex: &Mutex<Userland>) {
        extern "C" {
            fn jump_usermode(process_base_address: u64, stack_top_address: u64, entry_address: u64);
        }

        unsafe {
            self.current_process = 0;

            self.processes[0].launch();
            self.processes[1].launch();
            self.processes[2].launch();
            self.processes[3].launch();
            self.processes[0].activate(true);

            // FIXME this feels very wrong!
            mutex.force_unlock();

            jump_usermode(
                self.processes[0].get_c3_page_map_l4_base_address(),
                self.processes[0].get_stack_top_address(),
                self.processes[0].get_entry_ip(),
            );
        }
    }

    pub fn switch_process(&mut self) {
        // TODO for now scheduler is simply going round robin
        let last_process = self.current_process;

        loop {
            self.current_process += 1;
            if self.current_process == self.processes.len() {
                self.current_process = 0;
            }
            if self.processes[self.current_process].activatable() {
                break;
            }

            // not a single userspace process ready for execution
            if self.current_process == last_process {
                return;
            }
        }

        self.processes[last_process].passivate();
        self.processes[self.current_process].activate(false);
    }

    pub fn get_current_process_id(&self) -> usize {
        self.current_process
    }
}

// very simple scheduler
pub fn schedule() {
    USERLAND.lock().switch_process();
}
