use crate::kprint;
use core::arch::asm;

// stores a process' registers when it gets interrupted
#[derive(Default)]
struct registers_struct {
    rax: u64,
    rbx: u64,
    rcx: u64,
    rdx: u64,
    rsi: u64,
    rdi: u64,
    rbp: u64,
    rsp: u64,
    r8: u64,
    r9: u64,
    r10: u64,
    r11: u64,
    r12: u64,
    r13: u64,
    r14: u64,
    r15: u64,
    rip: u64,
    rfl: u64,
    cs: u64,
    ss: u64,
}

#[repr(C)]
#[repr(align(4096))]
struct PageTable {
    entry: [u64; 512],
}

impl PageTable {
    pub fn new() -> Self {
        let mut entries: [u64; 512] = [0; 512];
        // TODO start with providing only the upmost pages for a process stack (lower end to do)
        //entries[511] = 0b111; // present, writable, access from user
        Self { entry: entries }
    }
}

pub struct Process {
    registers: registers_struct,

    l2_page_directory_table: PageTable,
    l3_page_directory_pointer_table: PageTable,
    l4_page_map_l4_table: PageTable,
}

impl Process {
    pub fn new() -> Self {
        // Initialize paging
        let mut l2_page_directory_table: PageTable = PageTable::new();
        let mut l3_page_directory_pointer_table: PageTable = PageTable::new();
        let mut l4_page_map_l4_table: PageTable = PageTable::new();

        // TODO remove hard coding
        // Upper end of page which begins at 0x2000000 = 50 MByte in phys RAM
        // TODO only one page (2MB) yet!
        l2_page_directory_table.entry[511] = 0x2000000 | 0b10000111; // bitmask: present, writable, huge page, access from user
        l3_page_directory_pointer_table.entry[511] =
            &l2_page_directory_table as *const _ as u64 | 0b111;
        l4_page_map_l4_table.entry[511] =
            &l3_page_directory_pointer_table as *const _ as u64 | 0b111;

        // TODO Hack: map the kernel pages from main.asm to process
        // TODO Later, the kernel pages should be restructed to superuser access; in order to do so, the process code and data must be fully in userspace pages
        let mut cr3: u64;

        unsafe {
            asm!("mov {}, cr3", out(reg) cr3);
            l4_page_map_l4_table.entry[0] = *(cr3 as *const _);
        }

        Self {
            registers: Default::default(),
            l2_page_directory_table: l2_page_directory_table,
            l3_page_directory_pointer_table: l3_page_directory_pointer_table,
            l4_page_map_l4_table: l4_page_map_l4_table,
        }
    }

    pub fn getC3PageMapL4BaseAddress(&self) -> u64 {
        // According to AMD64 Volume 2 p. 146 only bits 13 to 51 are relevant for C3, but the rest seems (?) ignored
        &(self.l4_page_map_l4_table) as *const _ as u64
    }

    pub fn getStackTopAddress(&self) -> u64 {
        // Virtual Address, see AMD64 Volume 2 p. 146
        0xffff_ffff_ffff_ffff //3fff --> set 3*9 bits to 1 to identify each topmost entry in each table; fffff --> topmost address in the page; rest also 1 because sign extend
    }
}
