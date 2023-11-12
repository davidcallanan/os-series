// https://github.com/scprogramming/Jazz2.0/blob/main/src/interrupts/idt.c
// https://wiki.osdev.org/Interrupts_Tutorial

use crate::logging;
use core::arch::asm;

#[repr(C, packed(2))]
#[derive(Clone, Copy)]
struct IdtEntryStruct {
    base_low: u16,
    sel: u16,
    always0: u8,
    flags: u8,
    base_mid: u16,
    base_high: u32,
    reserved: u32,
}

#[repr(C)]
#[repr(packed(2))]
pub struct IdtPtrStruct {
    pub limit: u16,
    pub base: u64,
}

static mut IDT_ENTRIES: [IdtEntryStruct; 256] = [IdtEntryStruct {
    always0: 0,
    base_high: 0,
    base_mid: 0,
    base_low: 0,
    flags: 0,
    sel: 0,
    reserved: 0,
}; 256];

#[repr(C)]
#[repr(packed(2))]
#[derive(Debug)]
// TODO Requires 64 bit types, needs more checking/testing
pub struct InterruptRegisters {
    cr2: u64,
    ds: u64,
    rdi: u64,
    rsi: u64,
    rbp: u64,
    rsp: u64,
    rbx: u64,
    rdx: u64,
    rcx: u64,
    rax: u64,
    int_no: u64,
    err_code: u64,
    rip: u64,
    csm: u64,
    eflags: u64,
    useresp: u64,
    ss: u64,
}

//https://github.com/rust-osdev/x86_64/blob/d891bdbbb1fc8e41987309685829e28d1ec305e4/src/structures/idt.rs#L941
#[repr(C)]
#[derive(Debug)]
pub struct StackFrame {
    pub instruction_pointer: u64,
    /// The code segment selector, padded with zeros.
    pub code_segment: u64,
    /// The flags register before the interrupt handler was invoked.
    pub cpu_flags: u64,
    /// The stack pointer at the time of the interrupt.
    pub stack_pointer: u64,
    /// The stack segment descriptor at the time of the interrupt (often zero in 64-bit mode).
    pub stack_segment: u64,
}

#[no_mangle]
pub extern "C" fn isr_handler(/*regs: InterruptRegisters, stackframe: StackFrame*/) {
    //if regs.int_no < 32 {
    //println!(exception_messages[regs->int_no]);
    //println!("\n");
    //panic!("Exception! System Halted");
    logging::log("ISR");
    //print_line!("{:x?}", regs);
    //print_line!("{:x?}", stackframe);

    //out_port_b(0x20, 0x20);
    //}
}

#[no_mangle]
pub extern "C" fn irq_handler(/*regs: InterruptRegisters, stackframe: StackFrame*/) {
    /*
    void (*handler)(struct InterruptRegisters *regs);

    handler = irq_routines[regs->int_no - 32];

    if (handler){
        handler(regs);
    }

    if (regs->int_no >= 40){
        out_port_b(0xA0, 0x20);
    }
    */

    logging::log("IRQ");
    //print_line!("{:x?}", regs);
    //print_line!("{:x?}", stackframe);
    out_port_b(0x20, 0x20);
}

fn out_port_b(port: u16, value: u8) {
    unsafe {
        asm!(
            r#"out %al, %dx"#,
            in("dx") port,
            in("al") value,
            options(att_syntax)
        );
    }
}

fn set_idt_gate(num: usize, base: u64, sel: u16, flags: u8) {
    unsafe {
        IDT_ENTRIES[num].base_low = (base & 0xFFFF) as u16;
        IDT_ENTRIES[num].base_mid = ((base >> 16) & 0xFFFF) as u16;
        IDT_ENTRIES[num].base_high = ((base >> 32) & 0xFFFFFFFF) as u32;
        IDT_ENTRIES[num].sel = sel;
        IDT_ENTRIES[num].always0 = 0;
        IDT_ENTRIES[num].flags = flags; // | 0x60;
        IDT_ENTRIES[num].reserved = 0;
    }
}

/*
// could work with nightly
// https://doc.rust-lang.org/src/core/macros/mod.rs.html#1009
macro_rules! setIdtGate {
    ($num: expr) => {
        extern "C" { fn isr$num();    }

        let isr$num: unsafe extern "C" fn() = isr$num;
        let addr_isr$num = isr0 as u64;

        setIdtGate_($num, addr_isr$num, 0x08, 0x8E);
    };
}
*/

pub fn init_idt() {
    //0x20 commands and 0x21 data
    //0xA0 commands and 0xA1 data

    out_port_b(0x20, 0x11);
    out_port_b(0xA0, 0x11);

    out_port_b(0x21, 0x20);
    out_port_b(0xA1, 0x28);

    out_port_b(0x21, 0x04);
    out_port_b(0xA1, 0x02);

    out_port_b(0x21, 0x01);
    out_port_b(0xA1, 0x01);

    out_port_b(0x21, 0x0);
    out_port_b(0xA1, 0x0);

    // Only keystrokes
    //https://wiki.osdev.org/I_Can%27t_Get_Interrupts_Working#IRQ_problems
    //out_port_b(0x21, 0xfd);
    //out_port_b(0xa1, 0xff);

    // Generated with http://www.mynikko.com/tools/tool_incrementstr.html
    extern "C" {
        fn isr0();
    }
    let isr0: unsafe extern "C" fn() = isr0;
    let addr_isr0 = isr0 as u64;
    set_idt_gate(0, addr_isr0, 0x08, 0x8E);

    extern "C" {
        fn isr1();
    }
    let isr1: unsafe extern "C" fn() = isr1;
    let addr_isr1 = isr1 as u64;
    set_idt_gate(1, addr_isr1, 0x08, 0x8E);

    extern "C" {
        fn isr2();
    }
    let isr2: unsafe extern "C" fn() = isr2;
    let addr_isr2 = isr2 as u64;
    set_idt_gate(2, addr_isr2, 0x08, 0x8E);

    extern "C" {
        fn isr3();
    }
    let isr3: unsafe extern "C" fn() = isr3;
    let addr_isr3 = isr3 as u64;
    set_idt_gate(3, addr_isr3, 0x08, 0x8E);

    extern "C" {
        fn isr4();
    }
    let isr4: unsafe extern "C" fn() = isr4;
    let addr_isr4 = isr4 as u64;
    set_idt_gate(4, addr_isr4, 0x08, 0x8E);

    extern "C" {
        fn isr5();
    }
    let isr5: unsafe extern "C" fn() = isr5;
    let addr_isr5 = isr5 as u64;
    set_idt_gate(5, addr_isr5, 0x08, 0x8E);

    extern "C" {
        fn isr6();
    }
    let isr6: unsafe extern "C" fn() = isr6;
    let addr_isr6 = isr6 as u64;
    set_idt_gate(6, addr_isr6, 0x08, 0x8E);

    extern "C" {
        fn isr7();
    }
    let isr7: unsafe extern "C" fn() = isr7;
    let addr_isr7 = isr7 as u64;
    set_idt_gate(7, addr_isr7, 0x08, 0x8E);

    extern "C" {
        fn isr8();
    }
    let isr8: unsafe extern "C" fn() = isr8;
    let addr_isr8 = isr8 as u64;
    set_idt_gate(8, addr_isr8, 0x08, 0x8E);

    extern "C" {
        fn isr9();
    }
    let isr9: unsafe extern "C" fn() = isr9;
    let addr_isr9 = isr9 as u64;
    set_idt_gate(9, addr_isr9, 0x08, 0x8E);

    extern "C" {
        fn isr10();
    }
    let isr10: unsafe extern "C" fn() = isr10;
    let addr_isr10 = isr10 as u64;
    set_idt_gate(10, addr_isr10, 0x08, 0x8E);

    extern "C" {
        fn isr11();
    }
    let isr11: unsafe extern "C" fn() = isr11;
    let addr_isr11 = isr11 as u64;
    set_idt_gate(11, addr_isr11, 0x08, 0x8E);

    extern "C" {
        fn isr12();
    }
    let isr12: unsafe extern "C" fn() = isr12;
    let addr_isr12 = isr12 as u64;
    set_idt_gate(12, addr_isr12, 0x08, 0x8E);

    extern "C" {
        fn isr13();
    }
    let isr13: unsafe extern "C" fn() = isr13;
    let addr_isr13 = isr13 as u64;
    set_idt_gate(13, addr_isr13, 0x08, 0x8E);

    extern "C" {
        fn isr14();
    }
    let isr14: unsafe extern "C" fn() = isr14;
    let addr_isr14 = isr14 as u64;
    set_idt_gate(14, addr_isr14, 0x08, 0x8E);

    extern "C" {
        fn isr15();
    }
    let isr15: unsafe extern "C" fn() = isr15;
    let addr_isr15 = isr15 as u64;
    set_idt_gate(15, addr_isr15, 0x08, 0x8E);

    extern "C" {
        fn isr16();
    }
    let isr16: unsafe extern "C" fn() = isr16;
    let addr_isr16 = isr16 as u64;
    set_idt_gate(16, addr_isr16, 0x08, 0x8E);

    extern "C" {
        fn isr17();
    }
    let isr17: unsafe extern "C" fn() = isr17;
    let addr_isr17 = isr17 as u64;
    set_idt_gate(17, addr_isr17, 0x08, 0x8E);

    extern "C" {
        fn isr18();
    }
    let isr18: unsafe extern "C" fn() = isr18;
    let addr_isr18 = isr18 as u64;
    set_idt_gate(18, addr_isr18, 0x08, 0x8E);

    extern "C" {
        fn isr19();
    }
    let isr19: unsafe extern "C" fn() = isr19;
    let addr_isr19 = isr19 as u64;
    set_idt_gate(19, addr_isr19, 0x08, 0x8E);

    extern "C" {
        fn isr20();
    }
    let isr20: unsafe extern "C" fn() = isr20;
    let addr_isr20 = isr20 as u64;
    set_idt_gate(20, addr_isr20, 0x08, 0x8E);

    extern "C" {
        fn isr21();
    }
    let isr21: unsafe extern "C" fn() = isr21;
    let addr_isr21 = isr21 as u64;
    set_idt_gate(21, addr_isr21, 0x08, 0x8E);

    extern "C" {
        fn isr22();
    }
    let isr22: unsafe extern "C" fn() = isr22;
    let addr_isr22 = isr22 as u64;
    set_idt_gate(22, addr_isr22, 0x08, 0x8E);

    extern "C" {
        fn isr23();
    }
    let isr23: unsafe extern "C" fn() = isr23;
    let addr_isr23 = isr23 as u64;
    set_idt_gate(23, addr_isr23, 0x08, 0x8E);

    extern "C" {
        fn isr24();
    }
    let isr24: unsafe extern "C" fn() = isr24;
    let addr_isr24 = isr24 as u64;
    set_idt_gate(24, addr_isr24, 0x08, 0x8E);

    extern "C" {
        fn isr25();
    }
    let isr25: unsafe extern "C" fn() = isr25;
    let addr_isr25 = isr25 as u64;
    set_idt_gate(25, addr_isr25, 0x08, 0x8E);

    extern "C" {
        fn isr26();
    }
    let isr26: unsafe extern "C" fn() = isr26;
    let addr_isr26 = isr26 as u64;
    set_idt_gate(26, addr_isr26, 0x08, 0x8E);

    extern "C" {
        fn isr27();
    }
    let isr27: unsafe extern "C" fn() = isr27;
    let addr_isr27 = isr27 as u64;
    set_idt_gate(27, addr_isr27, 0x08, 0x8E);

    extern "C" {
        fn isr28();
    }
    let isr28: unsafe extern "C" fn() = isr28;
    let addr_isr28 = isr28 as u64;
    set_idt_gate(28, addr_isr28, 0x08, 0x8E);

    extern "C" {
        fn isr29();
    }
    let isr29: unsafe extern "C" fn() = isr29;
    let addr_isr29 = isr29 as u64;
    set_idt_gate(29, addr_isr29, 0x08, 0x8E);

    extern "C" {
        fn isr30();
    }
    let isr30: unsafe extern "C" fn() = isr30;
    let addr_isr30 = isr30 as u64;
    set_idt_gate(30, addr_isr30, 0x08, 0x8E);

    extern "C" {
        fn isr31();
    }
    let isr31: unsafe extern "C" fn() = isr31;
    let addr_isr31 = isr31 as u64;
    set_idt_gate(31, addr_isr31, 0x08, 0x8E);

    extern "C" {
        fn irq0();
    }
    let irq0: unsafe extern "C" fn() = irq0;
    let addr_irq0 = irq0 as u64;
    set_idt_gate(32, addr_irq0, 0x08, 0x8E);

    extern "C" {
        fn irq1();
    }
    let irq1: unsafe extern "C" fn() = irq1;
    let addr_irq1 = irq1 as u64;
    set_idt_gate(33, addr_irq1, 0x08, 0x8E);

    extern "C" {
        fn irq2();
    }
    let irq2: unsafe extern "C" fn() = irq2;
    let addr_irq2 = irq2 as u64;
    set_idt_gate(34, addr_irq2, 0x08, 0x8E);

    extern "C" {
        fn irq3();
    }
    let irq3: unsafe extern "C" fn() = irq3;
    let addr_irq3 = irq3 as u64;
    set_idt_gate(35, addr_irq3, 0x08, 0x8E);

    extern "C" {
        fn irq4();
    }
    let irq4: unsafe extern "C" fn() = irq4;
    let addr_irq4 = irq4 as u64;
    set_idt_gate(36, addr_irq4, 0x08, 0x8E);

    extern "C" {
        fn irq5();
    }
    let irq5: unsafe extern "C" fn() = irq5;
    let addr_irq5 = irq5 as u64;
    set_idt_gate(37, addr_irq5, 0x08, 0x8E);

    extern "C" {
        fn irq6();
    }
    let irq6: unsafe extern "C" fn() = irq6;
    let addr_irq6 = irq6 as u64;
    set_idt_gate(38, addr_irq6, 0x08, 0x8E);

    extern "C" {
        fn irq7();
    }
    let irq7: unsafe extern "C" fn() = irq7;
    let addr_irq7 = irq7 as u64;
    set_idt_gate(39, addr_irq7, 0x08, 0x8E);

    extern "C" {
        fn irq8();
    }
    let irq8: unsafe extern "C" fn() = irq8;
    let addr_irq8 = irq8 as u64;
    set_idt_gate(40, addr_irq8, 0x08, 0x8E);

    extern "C" {
        fn irq9();
    }
    let irq9: unsafe extern "C" fn() = irq9;
    let addr_irq9 = irq9 as u64;
    set_idt_gate(41, addr_irq9, 0x08, 0x8E);

    extern "C" {
        fn irq10();
    }
    let irq10: unsafe extern "C" fn() = irq10;
    let addr_irq10 = irq10 as u64;
    set_idt_gate(42, addr_irq10, 0x08, 0x8E);

    extern "C" {
        fn irq11();
    }
    let irq11: unsafe extern "C" fn() = irq11;
    let addr_irq11 = irq11 as u64;
    set_idt_gate(43, addr_irq11, 0x08, 0x8E);

    extern "C" {
        fn irq12();
    }
    let irq12: unsafe extern "C" fn() = irq12;
    let addr_irq12 = irq12 as u64;
    set_idt_gate(44, addr_irq12, 0x08, 0x8E);

    extern "C" {
        fn irq13();
    }
    let irq13: unsafe extern "C" fn() = irq13;
    let addr_irq13 = irq13 as u64;
    set_idt_gate(45, addr_irq13, 0x08, 0x8E);

    extern "C" {
        fn irq14();
    }
    let irq14: unsafe extern "C" fn() = irq14;
    let addr_irq14 = irq14 as u64;
    set_idt_gate(46, addr_irq14, 0x08, 0x8E);

    extern "C" {
        fn irq15();
    }
    let irq15: unsafe extern "C" fn() = irq15;
    let addr_irq15 = irq15 as u64;
    set_idt_gate(47, addr_irq15, 0x08, 0x8E);

    extern "C" {
        fn isr128();
    }
    let isr128: unsafe extern "C" fn() = isr128;
    let addr_isr128 = isr128 as u64;
    set_idt_gate(128, addr_isr128, 0x08, 0x8E);

    extern "C" {
        fn isr177();
    }
    let isr177: unsafe extern "C" fn() = isr177;
    let addr_isr177 = isr177 as u64;
    set_idt_gate(177, addr_isr177, 0x08, 0x8E);

    unsafe {
        let idt_ptr: IdtPtrStruct = IdtPtrStruct {
            limit: 128 * 256 - 1, //(core::mem::size_of::<IdtEntryStruct>() * 256 - 1) as u16,
            //https://stackoverflow.com/a/64311274
            // https://github.com/rust-osdev/x86_64/blob/master/src/addr.rs#L100C9-L100C9
            // Complexity from last link probably not required
            base: IDT_ENTRIES.as_ptr() as u64, //(((IDT_ENTRIES.as_ptr() as u64) << 16) as i64 >> 16) as u64,
        };
        asm!(
            "lidt [{}]
            sti",
            in(reg) &idt_ptr, options(readonly, nostack, preserves_flags)
        );
    }
}

/*

unsigned char* exception_messages[] = {
    "Division By Zero",
    "Debug",
    "Non Maskable Interrupt",
    "Breakpoint",
    "Into Detected Overflow",
    "Out of Bounds",
    "Invalid Opcode",
    "No Coprocessor",
    "Double fault",
    "Coprocessor Segment Overrun",
    "Bad TSS",
    "Segment not present",
    "Stack fault",
    "General protection fault",
    "Page fault",
    "Unknown Interrupt",
    "Coprocessor Fault",
    "Alignment Fault",
    "Machine Check",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved",
    "Reserved"
};



void *irq_routines[16] = {
    0,0,0,0,0,0,0,0,
    0,0,0,0,0,0,0,0
};

void irq_install_handler (int irq, void (*handler)(struct InterruptRegisters *r)){
    irq_routines[irq] = handler;
}

void irq_uninstall_handler(int irq){
    irq_routines[irq] = 0;
}

*/
