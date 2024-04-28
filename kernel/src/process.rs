use crate::kprint;
use core::arch::asm;
use core::ptr::addr_of;

static mut KERNEL_CR3: u64 = 0;

// stores a process' registers when it gets interrupted
#[repr(C)]
#[derive(Default)]
struct RegistersStruct {
    // Has to be always in sync with asm macro "pop_all_registers"
    r15: u64,
    r14: u64,
    r13: u64,
    r12: u64,
    r11: u64,
    r10: u64,
    r9: u64,
    r8: u64,
    rbp: u64,
    rdi: u64,
    rsi: u64,
    rdx: u64,
    rcx: u64,
    rbx: u64,
    rax: u64,
}

#[repr(C)]
#[repr(align(4096))]
struct PageTable {
    entry: [u64; 512],
}

impl PageTable {
    fn default() -> Self {
        Self { entry: [0; 512] }
    }
}

// TODO make more elegant
// available memory in qemu by default is 128 MByte (2^27); we are using 2 MByte page frames (2^21) -> 2^(27-21) = 64

const MAX_PAGE_FRAMES: usize = 64;
static mut AVAILABLE_MEMORY: [bool; MAX_PAGE_FRAMES] = {
    let mut array = [false; MAX_PAGE_FRAMES];

    // some page frames are already allocated in main.asm -> setup_page_tables
    array[0] = true;
    array[1] = true;
    array[2] = true;
    array[3] = true;
    array[4] = true;
    array[5] = true;
    array[6] = true;
    array[7] = true;
    array[8] = true;
    array[9] = true;

    // TODO Stack for interrupts, see HackID1
    array[10] = true;
    array
};

fn allocate_page_frame() -> u64 {
    // TODO make safe
    // TODO make faster by not iterating instead storing next free page frame
    unsafe {
        for i in 0..MAX_PAGE_FRAMES - 1 {
            if AVAILABLE_MEMORY[i] == false {
                AVAILABLE_MEMORY[i] = true;
                return i as u64 * 0x200000 as u64;
            }
        }
    }

    return 0;
}

fn _print_page_table_tree_for_cr3() {
    let mut cr3: u64;

    unsafe {
        asm!("mov r12, cr3", out("r12") cr3);
    }

    print_page_table_tree(cr3);
}

fn check_half(entry: *const u64) -> *const u64 {
    if entry < 0xffff800000000000 as *const u64 {
        return (entry as u64 + 0xffff800000000000 as u64) as *const u64;
    }
    entry
}

fn print_page_table_tree(start_addr: u64) {
    let entry_mask = 0x0008_ffff_ffff_f800;

    unsafe {
        kprint!("start_addr: {:#x}\n", start_addr);

        for l4_entry in 0..512 {
            let l4bits = *check_half((start_addr + l4_entry * 8) as *const u64);
            if l4bits != 0 {
                kprint!("   L4: {} - {:#x}\n", l4_entry, l4bits & entry_mask);

                for l3_entry in 0..512 {
                    let l3bits = *check_half(((l4bits & entry_mask) + l3_entry * 8) as *const u64);
                    if l3bits != 0 {
                        kprint!("      L3: {} - {:#x}\n", l3_entry, l3bits & entry_mask);

                        for l2_entry in 0..512 {
                            let l2bits =
                                *check_half(((l3bits & entry_mask) + l2_entry * 8) as *const u64);

                            if l2bits != 0 {
                                kprint!("         L2: {} - {:#x}\n", l2_entry, l2bits & entry_mask);
                            }
                        }
                    }
                }
            }
        }
    }
}

enum ProcessState {
    New,
    Prepared,
    Active,
    Passive,
}

pub struct Process {
    registers: RegistersStruct,

    l2_page_directory_table: PageTable,
    l3_page_directory_pointer_table: PageTable,
    l4_page_map_l4_table: PageTable,

    l2_page_directory_table_beginning: PageTable,
    l3_page_directory_pointer_table_beginning: PageTable,

    rip: u64,
    rsp: u64,
    cr3: u64,
    ss: u64,
    cs: u64,
    rflags: u64,

    state: ProcessState,
}

impl Process {
    pub fn new() -> Self {
        Self {
            registers: RegistersStruct::default(),
            l2_page_directory_table: PageTable::default(),
            l2_page_directory_table_beginning: PageTable::default(),
            l3_page_directory_pointer_table: PageTable::default(),
            l3_page_directory_pointer_table_beginning: PageTable::default(),
            l4_page_map_l4_table: PageTable::default(),

            rip: 0,
            cr3: 0,
            ss: 0x1b,
            cs: 0x23,
            rflags: 0x202,
            rsp: 0,
            state: ProcessState::New,
        }
    }

    pub fn initialize(&mut self) {
        // TODO remove hard coding
        // TODO Task stack
        // Upper end of page which begins at 0x2000000 = 50 MByte in phys RAM
        // TODO only one page (2MB) yet!
        self.l2_page_directory_table.entry[511] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user

        // TODO HackID1: Fixed kernel stack for interrupts (starts at 20 MByte)
        self.l2_page_directory_table.entry[510] = 10 * 0x200000 | 0b10000011; // bitmask: present, writable, huge page

        self.l3_page_directory_pointer_table.entry[511] =
            Process::get_physical_address_for_virtual_address(
                &self.l2_page_directory_table as *const _ as u64,
            ) | 0b111;
        self.l4_page_map_l4_table.entry[511] = Process::get_physical_address_for_virtual_address(
            &self.l3_page_directory_pointer_table as *const _ as u64,
        ) | 0b111;

        // allocate two pages page at beginning of virtual memory for elf loading
        // TODO allocate more if needed

        self.l2_page_directory_table_beginning.entry[0] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user
        self.l2_page_directory_table_beginning.entry[1] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user
        self.l3_page_directory_pointer_table_beginning.entry[0] =
            Process::get_physical_address_for_virtual_address(
                &self.l2_page_directory_table_beginning as *const _ as u64,
            ) | 0b111;
        self.l4_page_map_l4_table.entry[0] = Process::get_physical_address_for_virtual_address(
            &self.l3_page_directory_pointer_table_beginning as *const _ as u64,
        ) | 0b111;

        // TODO Hack? map the kernel pages from main.asm to process
        // TODO Later, the kernel pages should be restructed to superuser access; in order to do so, the process code and data must be fully in userspace pages
        unsafe {
            if KERNEL_CR3 == 0 {
                asm!("mov r15, cr3", out("r15") KERNEL_CR3);
            }

            kprint!("Kernel CR3: {:x}\n", KERNEL_CR3);

            self.l4_page_map_l4_table.entry[256] = *((KERNEL_CR3 + 256 * 8) as *const _);
        }

        // TODO Here we load the new pagetable into cr3 for the first process. This needs to happen because otherwise we cant load the programm into the first pages. This is a hack I think
        self.cr3 = Process::get_physical_address_for_virtual_address(
            &self.l4_page_map_l4_table as *const _ as u64,
        );

        kprint!("Process CR3: {:x}\n", self.cr3);

        unsafe {
            print_page_table_tree(KERNEL_CR3 as u64);
        }
        // FIXME THIS IS BROKEN!

        unsafe {
            asm!(
                "mov cr3, r15",
                in("r15") self.cr3,
                options(nostack, preserves_flags)
            );
        }

        print_page_table_tree(&self.l4_page_map_l4_table as *const _ as u64);

        self.rsp = 0xffff_ffff_ffff_ffff;

        self.rip = Process::load_elf_from_bin();

        unsafe {
            asm!(
                "mov cr3, r15",
                in("r15") KERNEL_CR3,
                options(nostack, preserves_flags)
            );
        }

        self.ss = 0x1b;
        self.cs = 0x23;
        self.rflags = 0x202;
        self.state = ProcessState::Prepared;
    }

    pub fn launch(&mut self) {
        self.state = ProcessState::Passive;
    }

    pub fn activate(&mut self, initial_start: bool) {
        extern "C" {
            static mut pushed_registers: *mut RegistersStruct;
            static mut stack_frame: *mut u64;
        }

        unsafe {
            kprint!("Stack frame: {:x}\n", stack_frame as u64);
            kprint!("Pushed registers: {:x}\n", pushed_registers as u64);

            if !initial_start {
                (*pushed_registers).r15 = self.registers.r15;
                (*pushed_registers).r14 = self.registers.r14;
                (*pushed_registers).r13 = self.registers.r13;
                (*pushed_registers).r12 = self.registers.r12;
                (*pushed_registers).r11 = self.registers.r11;
                (*pushed_registers).r10 = self.registers.r10;
                (*pushed_registers).r9 = self.registers.r9;
                (*pushed_registers).r8 = self.registers.r8;
                (*pushed_registers).rbp = self.registers.rbp;
                (*pushed_registers).rsi = self.registers.rsi;
                (*pushed_registers).rdx = self.registers.rdx;
                (*pushed_registers).rcx = self.registers.rcx;
                (*pushed_registers).rbx = self.registers.rbx;
                (*pushed_registers).rax = self.registers.rax;

                core::ptr::write_volatile(stack_frame.add(0), self.rip);
                core::ptr::write_volatile(stack_frame.add(1), self.cs);
                core::ptr::write_volatile(stack_frame.add(2), self.rflags);
                core::ptr::write_volatile(stack_frame.add(3), self.rsp);
                core::ptr::write_volatile(stack_frame.add(4), self.ss);
            }

            //  HIER!!!!!!!!
            //schreibe zwar was in den Stack, aber dann lade ich per cr3 ja neues paging!!!!

            // x /20xg 0xffffffffffcfffb8
            asm!(
                "mov cr3, r15",
                in("r15") self.cr3,
                options(nostack, preserves_flags),
                clobber_abi("C")
            );

            //TSS_ENTRY.rsp0 = self.get_tss_rsp0();
        }

        self.state = ProcessState::Active;
    }

    pub fn passivate(&mut self) {
        extern "C" {
            static pushed_registers: *const RegistersStruct;
            static stack_frame: *const u64;
        }

        unsafe {
            //kprint!("Stack frame: {:x}\n", stack_frame as u64);

            self.registers.r15 = (*pushed_registers).r15;
            self.registers.r14 = (*pushed_registers).r14;
            self.registers.r13 = (*pushed_registers).r13;
            self.registers.r12 = (*pushed_registers).r12;
            self.registers.r11 = (*pushed_registers).r11;
            self.registers.r10 = (*pushed_registers).r10;
            self.registers.r9 = (*pushed_registers).r9;
            self.registers.r8 = (*pushed_registers).r8;
            self.registers.rbp = (*pushed_registers).rbp;
            self.registers.rsi = (*pushed_registers).rsi;
            self.registers.rdx = (*pushed_registers).rdx;
            self.registers.rcx = (*pushed_registers).rcx;
            self.registers.rbx = (*pushed_registers).rbx;
            self.registers.rax = (*pushed_registers).rax;

            self.rip = *(stack_frame.add(0));
            self.cs = *(stack_frame.add(1));
            self.rflags = *(stack_frame.add(2));
            self.rsp = *(stack_frame.add(3));
            self.ss = *(stack_frame.add(4));
        }

        self.state = ProcessState::Passive;
    }

    pub fn activatable(&self) -> bool {
        match self.state {
            ProcessState::Passive => true,
            _ => false,
        }
    }

    fn _get_tss_rsp0(&self) -> u64 {
        0xffff_ffff_ffcf_ffff
    }

    // According to AMD Volume 2, page 146
    fn get_physical_address_for_virtual_address(vaddr: u64) -> u64 {
        // Simple variant, only works for kernel memory
        vaddr - 0xffff800000000000

        // TODO get this running
        /*let page_map_l4_table_offset = (vaddr & 0x0000_ff80_0000_0000) >> 38;
        let page_directory_pointer_offset = (vaddr & 0x0000_007f_f000_0000) >> 29;
        let page_directory_offset = (vaddr & 0x0000_000_ff80_0000) >> 20;
        let page_offset = vaddr & 0x0000_000_007f_ffff;

        unsafe {
            let mut cr3: u64;

            asm!("mov {}, cr3", out(reg) cr3);

            let page_map_l4_base_address = cr3 & 0x0008_ffff_ffff_f800;

            let entry_mask = 0x0008_ffff_ffff_f800;

            let page_directory_pointer_table_address =
                *((page_map_l4_base_address + page_map_l4_table_offset * 8) as *const u64)
                    & entry_mask;

            let page_directory_table_address = *((page_directory_pointer_table_address
                + page_directory_pointer_offset * 8)
                as *const u64)
                & entry_mask;

            let physical_page_address = *((page_directory_table_address + page_directory_offset * 8)
                as *const u64)
                & entry_mask;

            return *((physical_page_address + page_offset) as *const u64);
        }*/
    }

    pub fn get_c3_page_map_l4_base_address(&self) -> u64 {
        Process::get_physical_address_for_virtual_address(
            &(self.l4_page_map_l4_table) as *const _ as u64,
        )
    }

    pub fn get_stack_top_address(&self) -> u64 {
        // Virtual Address, see AMD64 Volume 2 p. 146
        0xffff_ffff_ffff_ffff //3fff --> set 3*9 bits to 1 to identify each topmost entry in each table; fffff --> topmost address in the page; rest also 1 because sign extend
    }

    pub fn get_entry_ip(&self) -> u64 {
        self.rip
    }

    pub fn load_elf_from_bin() -> u64 {
        extern "C" {
            static mut _binary_build_userspace_x86_64_unknown_none_debug_helloworld_start: u8;
            static mut _binary_build_userspace_x86_64_unknown_none_debug_helloworld_end: u8;
        }

        unsafe {
            kprint!(
                "embedded elf file\nstart: {:x}\n  end: {:x}\n",
                addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start)
                    as *const u8 as usize,
                addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_end)
                    as *const u8 as usize
            );

            let size = addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_end)
                as *const u8 as usize
                - addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start)
                    as *const u8 as usize;

            let slice = core::slice::from_raw_parts(
                addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start),
                size,
            );

            use elf::abi::PT_LOAD;
            use elf::endian::AnyEndian;

            let file = elf::ElfBytes::<AnyEndian>::minimal_parse(slice).expect("Open test1");

            let elf_header = file.ehdr;

            kprint!("Entry point is at: {:x}\n", elf_header.e_entry);

            let program_headers = file
                .segments()
                .unwrap()
                .iter()
                .filter(|phdr| phdr.p_type == PT_LOAD);

            for phdr in program_headers {
                kprint!(
                    "Load segment is at: {:x}\nMem Size is: {:x}\n",
                    phdr.p_vaddr,
                    phdr.p_memsz
                );

                asm!(
                    "mov rcx, {}
                    mov rsi, {}
                    mov rdi, {}
                    rep movsb",
                    in(reg) phdr.p_memsz,
                    in(reg) addr_of!(_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start) as *const u8 as usize + phdr.p_offset as usize,
                    in(reg) phdr.p_vaddr,
                    out("rcx") _,
                    out("rsi") _,
                    out("rdi") _
                )
            }

            elf_header.e_entry
        }
    }
}
