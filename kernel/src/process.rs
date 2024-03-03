use crate::kprint;
use core::arch::asm;

pub static mut CURRENT_PROCESS: u64 = core::u64::MAX;

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

fn print_page_table_tree_for_cr3() {
    let mut cr3: u64;

    unsafe {
        asm!("mov {}, cr3", out(reg) cr3);
    }

    print_page_table_tree(cr3);
}

fn print_page_table_tree(start_addr: u64) {
    let entry_mask = 0x0008_ffff_ffff_f800;

    unsafe {
        kprint!("start_addr: {:#x}\n", start_addr);

        for l4_entry in 0..512 {
            let l4bits = *((start_addr + l4_entry * 8) as *const u64);
            if l4bits != 0 {
                kprint!("   L4: {} - {:#x}\n", l4_entry, l4bits & entry_mask);

                for l3_entry in 0..512 {
                    let l3bits = *(((l4bits & entry_mask) + l3_entry * 8) as *const u64);
                    if l3bits != 0 {
                        kprint!("      L3: {} - {:#x}\n", l3_entry, l3bits & entry_mask);

                        for l2_entry in 0..512 {
                            let l2bits = *(((l3bits & entry_mask) + l2_entry * 8) as *const u64);

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

pub struct Process {
    registers: registers_struct,

    l2_page_directory_table: PageTable,
    l3_page_directory_pointer_table: PageTable,
    l4_page_map_l4_table: PageTable,

    entry_ip: u64,
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
        l2_page_directory_table.entry[511] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user

        // TODO Hack: Map video memory to virtual memory
        l2_page_directory_table.entry[510] = 0x0 | 0b10000011; // bitmask: present, writable, huge page, access from user

        l3_page_directory_pointer_table.entry[511] =
            Process::get_physical_address_for_virtual_address(
                &l2_page_directory_table as *const _ as u64,
            ) | 0b111;
        l4_page_map_l4_table.entry[511] = Process::get_physical_address_for_virtual_address(
            &l3_page_directory_pointer_table as *const _ as u64,
        ) | 0b111;

        // TODO Hack? map the kernel pages from main.asm to process
        // TODO Later, the kernel pages should be restructed to superuser access; in order to do so, the process code and data must be fully in userspace pages
        unsafe {
            let mut cr3: u64;

            asm!("mov {}, cr3", out(reg) cr3);

            l4_page_map_l4_table.entry[256] = *((cr3 + 256 * 8) as *const _);
        }

        // allocate two pages page at beginning of virtual memory for elf loading
        // TODO allocate more if needed
        let mut l2_page_directory_table_beginning: PageTable = PageTable::new();
        let mut l3_page_directory_pointer_table_beginning: PageTable = PageTable::new();

        l2_page_directory_table_beginning.entry[0] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user
        l2_page_directory_table_beginning.entry[1] = allocate_page_frame() | 0b10000111; // bitmask: present, writable, huge page, access from user
        l3_page_directory_pointer_table_beginning.entry[0] =
            Process::get_physical_address_for_virtual_address(
                &l2_page_directory_table_beginning as *const _ as u64,
            ) | 0b111;
        l4_page_map_l4_table.entry[0] = Process::get_physical_address_for_virtual_address(
            &l3_page_directory_pointer_table_beginning as *const _ as u64,
        ) | 0b111;

        print_page_table_tree(Process::get_physical_address_for_virtual_address(
            &l4_page_map_l4_table as *const _ as u64,
        ));

        unsafe {
            asm!(
                "mov cr3, {}",
                in(reg) Process::get_physical_address_for_virtual_address(&l4_page_map_l4_table as *const _ as u64)
            );
        }

        Self {
            registers: Default::default(),
            l2_page_directory_table: l2_page_directory_table,
            l3_page_directory_pointer_table: l3_page_directory_pointer_table,
            l4_page_map_l4_table: l4_page_map_l4_table,
            entry_ip: Process::load_elf_from_bin(),
        }
    }

    pub fn launch() {}

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
        self.entry_ip
    }

    pub fn load_elf_from_bin() -> u64 {
        extern "C" {
            static mut _binary_build_userspace_x86_64_unknown_none_debug_helloworld_start: u8;
            static mut _binary_build_userspace_x86_64_unknown_none_debug_helloworld_end: u8;
        }

        unsafe {
            kprint!(
                "embedded elf file\nstart: {:x}\n  end: {:x}\n",
                &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start as *const u8
                    as usize,
                &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_end as *const u8
                    as usize
            );

            let size = &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_end
                as *const u8 as usize
                - &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start as *const u8
                    as usize;

            let slice = core::slice::from_raw_parts(
                &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start,
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
                    in(reg) &_binary_build_userspace_x86_64_unknown_none_debug_helloworld_start as *const u8 as usize + phdr.p_offset as usize,
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
