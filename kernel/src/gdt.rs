use crate::logging;
use core::arch::asm;
use core::arch::global_asm;
use core::mem;
use core::ptr::addr_of;

// https://wiki.osdev.org/GDT_Tutorial
// https://en.wikipedia.org/wiki/Global_Descriptor_Table#GDT_in_64-bit
// https://blog.llandsmeer.com/tech/2019/07/21/uefi-x64-userland.html
// http://www.osdever.net/bkerndev/Docs/gdt.htm
// http://tuttlem.github.io/2014/07/11/a-gdt-primer.html

global_asm!(include_str!("gdt.S"));

#[repr(C)]
#[repr(packed)]
struct GDT {
    limit: u32,
    base: u32,
    access_byte: u8,
    flags: u8,
}

// https://wiki.osdev.org/GDT_Tutorial#Flat_.2F_Long_Mode_Setup
static mut GDT_ENTRIES: [[u8; 8]; 7] = [[0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8], [0; 8]];

// https://wiki.osdev.org/TSS#Long_Mode
#[repr(C)]
#[repr(packed(2))]
#[derive(Clone, Copy)]
pub struct Tss {
    pub reserved1: u32,
    pub rsp0: u64,
    pub rsp1: u64,
    pub rsp2: u64,
    pub reserved2: u64,
    pub ist1: u64,
    pub ist2: u64,
    pub ist3: u64,
    pub ist4: u64,
    pub ist5: u64,
    pub ist6: u64,
    pub ist7: u64,
    pub reserved3: u64,
    pub reserved4: u16,
    pub iopb: u16,
}

pub static mut TSS_ENTRY: Tss = Tss {
    reserved1: 0x0,
    rsp0: 0xffff_ffff_ffcf_ffff,
    rsp1: 0x0,
    rsp2: 0x0,
    reserved2: 0x0,
    ist1: 0x0,
    ist2: 0x0,
    ist3: 0x0,
    ist4: 0x0,
    ist5: 0x0,
    ist6: 0x0,
    ist7: 0x0,
    reserved3: 0x0,
    reserved4: 0x0,
    iopb: 0x0,
};

#[repr(C)]
#[repr(packed(2))]
struct GdtPtrStruct {
    size: u16,
    offset: u64,
}

pub fn init_gdt() {
    unsafe {
        GDT_ENTRIES = [
            // Null descriptor
            encode_gdt_entry(GDT {
                base: 0x0,
                limit: 0x0,
                access_byte: 0x0,
                flags: 0x0,
            }),
            // Kernel Mode Code Segment
            encode_gdt_entry(GDT {
                base: 0x0,
                limit: 0xfffff,
                access_byte: 0x9a,
                flags: 0xa,
            }),
            //  Kernel Mode Data Segment
            encode_gdt_entry(GDT {
                base: 0x0,
                limit: 0xfffff,
                access_byte: 0x92,
                flags: 0xc,
            }),
            //  User Mode Data Segment
            encode_gdt_entry(GDT {
                base: 0x0,
                limit: 0xfffff,
                access_byte: 0xf2,
                flags: 0xc,
            }),
            //  User Mode Code Segment
            encode_gdt_entry(GDT {
                base: 0x0,
                limit: 0xfffff,
                access_byte: 0xfa,
                flags: 0xa,
            }),
            //  Task State Segment
            encode_gdt_entry(GDT {
                base: addr_of!(TSS_ENTRY) as *const _ as u32,
                limit: addr_of!(TSS_ENTRY) as *const _ as u32 + mem::size_of::<Tss>() as u32 - 1,
                access_byte: 0x89,
                flags: 0xc,
            }),
            //  Task State Segment, 2nd part --> special treatment for system segment descriptor in long mode
            encode_gdt_entry(GDT {
                base: (addr_of!(TSS_ENTRY) as *const _ as u64 >> 48) as u32,
                limit: (addr_of!(TSS_ENTRY) as *const _ as u64 >> 32) as u32,
                access_byte: 0x0,
                flags: 0x0,
            }),
        ]
    };
    unsafe {
        let gdt_ptr: GdtPtrStruct = GdtPtrStruct {
            size: (mem::size_of::<GDT>() * GDT_ENTRIES.len() - 1) as u16,
            //https://stackoverflow.com/a/64311274
            // https://github.com/rust-osdev/x86_64/blob/master/src/addr.rs#L100C9-L100C9
            // Complexity from last link probably not required
            offset: (((GDT_ENTRIES.as_ptr() as u64) << 16) as i64 >> 16) as u64,
        };
        asm!("cli");
        asm!(
            "lgdt [{}]", in(reg) &gdt_ptr, options(readonly, nostack, preserves_flags)
        );
        extern "C" {
            fn reloadSegments();
        }
        reloadSegments();
        // Load tss segment selector into task register
        // sixth 8-byte selector, symbolically OR-ed with 0 to set the RPL (requested privilege level).
        asm!(
            "mov ax, (5 * 8) | 0
            ltr ax"
        );
    }
}

// https://wiki.osdev.org/GDT_Tutorial#Filling_the_Table
fn encode_gdt_entry(source: GDT) -> [u8; 8] {
    let mut target: [u8; 8] = [0; 8];

    // Check the limit to make sure that it can be encoded
    if source.limit > 0xFFFFF {
        logging::log("GDT cannot encode limits larger than 0xFFFFF");
    }

    // Encode the limit
    target[0] = source.limit as u8 & 0xFF;
    target[1] = (source.limit >> 8) as u8 & 0xFF;
    target[6] = (source.limit >> 16) as u8 & 0x0F;

    // Encode the base
    target[2] = source.base as u8 & 0xFF;
    target[3] = (source.base >> 8) as u8 & 0xFF;
    target[4] = (source.base >> 16) as u8 & 0xFF;
    target[7] = (source.base >> 24) as u8 & 0xFF;

    // Encode the access byte
    target[5] = source.access_byte;

    // Encode the flags
    target[6] |= source.flags << 4;

    target
}
