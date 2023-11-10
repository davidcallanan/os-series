use core::arch::asm;

// __attribute__((packed));
#[repr(C)]
#[repr(packed)]
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

// __attribute__((packed));
#[repr(C)]
#[repr(packed)]
struct IdtPtrStruct {
    limit: usize,
    base: *const IdtEntryStruct,
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

static mut IDT_PTR: IdtPtrStruct = IdtPtrStruct {
    limit: core::mem::size_of::<IdtEntryStruct>() * 256 - 1,
    base: unsafe { IDT_ENTRIES.as_ptr() },
};

#[repr(C)]
pub struct InterruptRegisters {
    int_no: u8,
}

#[no_mangle]
pub extern "C" fn isr_handler(regs: InterruptRegisters) {
    if regs.int_no < 32 {
        //println!(exception_messages[regs->int_no]);
        //println!("\n");
        panic!("Exception! System Halted");
    }
}

#[no_mangle]
pub extern "C" fn irq_handler(_regs: InterruptRegisters) {}

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
        IDT_ENTRIES[num].base_mid = (base / 0xFFFF0000) as u16;
        IDT_ENTRIES[num].base_high = (base / 0xFFFFFFFF) as u32;
        IDT_ENTRIES[num].sel = sel;
        IDT_ENTRIES[num].always0 = 0;
        IDT_ENTRIES[num].flags = flags | 0x60;
    }
}

/*
// could work with nightly
// https://doc.rust-lang.org/src/core/macros/mod.rs.html#1009
macro_rules! setIdtGate {
    ($num: expr) => {
        extern "C" fn isr$num() {}

        let isr$num: unsafe extern "C" fn() = isr$num;
        let addr_isr$num = isr0 as usize as u64;

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

    // Generated with http://www.mynikko.com/tools/tool_incrementstr.html
    extern "C" fn isr0() {}
    let isr0: unsafe extern "C" fn() = isr0;
    let addr_isr0 = isr0 as usize as u64;
    set_idt_gate(1, addr_isr0, 0x08, 0x8E);

    extern "C" fn isr1() {}
    let isr1: unsafe extern "C" fn() = isr1;
    let addr_isr1 = isr1 as usize as u64;
    set_idt_gate(1, addr_isr1, 0x08, 0x8E);

    extern "C" fn isr2() {}
    let isr2: unsafe extern "C" fn() = isr2;
    let addr_isr2 = isr2 as usize as u64;
    set_idt_gate(1, addr_isr2, 0x08, 0x8E);

    extern "C" fn isr3() {}
    let isr3: unsafe extern "C" fn() = isr3;
    let addr_isr3 = isr3 as usize as u64;
    set_idt_gate(1, addr_isr3, 0x08, 0x8E);

    extern "C" fn isr4() {}
    let isr4: unsafe extern "C" fn() = isr4;
    let addr_isr4 = isr4 as usize as u64;
    set_idt_gate(1, addr_isr4, 0x08, 0x8E);

    extern "C" fn isr5() {}
    let isr5: unsafe extern "C" fn() = isr5;
    let addr_isr5 = isr5 as usize as u64;
    set_idt_gate(1, addr_isr5, 0x08, 0x8E);

    extern "C" fn isr6() {}
    let isr6: unsafe extern "C" fn() = isr6;
    let addr_isr6 = isr6 as usize as u64;
    set_idt_gate(1, addr_isr6, 0x08, 0x8E);

    extern "C" fn isr7() {}
    let isr7: unsafe extern "C" fn() = isr7;
    let addr_isr7 = isr7 as usize as u64;
    set_idt_gate(1, addr_isr7, 0x08, 0x8E);

    extern "C" fn isr8() {}
    let isr8: unsafe extern "C" fn() = isr8;
    let addr_isr8 = isr8 as usize as u64;
    set_idt_gate(1, addr_isr8, 0x08, 0x8E);

    extern "C" fn isr9() {}
    let isr9: unsafe extern "C" fn() = isr9;
    let addr_isr9 = isr9 as usize as u64;
    set_idt_gate(1, addr_isr9, 0x08, 0x8E);

    extern "C" fn isr10() {}
    let isr10: unsafe extern "C" fn() = isr10;
    let addr_isr10 = isr10 as usize as u64;
    set_idt_gate(1, addr_isr10, 0x08, 0x8E);

    extern "C" fn isr11() {}
    let isr11: unsafe extern "C" fn() = isr11;
    let addr_isr11 = isr11 as usize as u64;
    set_idt_gate(1, addr_isr11, 0x08, 0x8E);

    extern "C" fn isr12() {}
    let isr12: unsafe extern "C" fn() = isr12;
    let addr_isr12 = isr12 as usize as u64;
    set_idt_gate(1, addr_isr12, 0x08, 0x8E);

    extern "C" fn isr13() {}
    let isr13: unsafe extern "C" fn() = isr13;
    let addr_isr13 = isr13 as usize as u64;
    set_idt_gate(1, addr_isr13, 0x08, 0x8E);

    extern "C" fn isr14() {}
    let isr14: unsafe extern "C" fn() = isr14;
    let addr_isr14 = isr14 as usize as u64;
    set_idt_gate(1, addr_isr14, 0x08, 0x8E);

    extern "C" fn isr15() {}
    let isr15: unsafe extern "C" fn() = isr15;
    let addr_isr15 = isr15 as usize as u64;
    set_idt_gate(1, addr_isr15, 0x08, 0x8E);

    extern "C" fn isr16() {}
    let isr16: unsafe extern "C" fn() = isr16;
    let addr_isr16 = isr16 as usize as u64;
    set_idt_gate(1, addr_isr16, 0x08, 0x8E);

    extern "C" fn isr17() {}
    let isr17: unsafe extern "C" fn() = isr17;
    let addr_isr17 = isr17 as usize as u64;
    set_idt_gate(1, addr_isr17, 0x08, 0x8E);

    extern "C" fn isr18() {}
    let isr18: unsafe extern "C" fn() = isr18;
    let addr_isr18 = isr18 as usize as u64;
    set_idt_gate(1, addr_isr18, 0x08, 0x8E);

    extern "C" fn isr19() {}
    let isr19: unsafe extern "C" fn() = isr19;
    let addr_isr19 = isr19 as usize as u64;
    set_idt_gate(1, addr_isr19, 0x08, 0x8E);

    extern "C" fn isr20() {}
    let isr20: unsafe extern "C" fn() = isr20;
    let addr_isr20 = isr20 as usize as u64;
    set_idt_gate(1, addr_isr20, 0x08, 0x8E);

    extern "C" fn isr21() {}
    let isr21: unsafe extern "C" fn() = isr21;
    let addr_isr21 = isr21 as usize as u64;
    set_idt_gate(1, addr_isr21, 0x08, 0x8E);

    extern "C" fn isr22() {}
    let isr22: unsafe extern "C" fn() = isr22;
    let addr_isr22 = isr22 as usize as u64;
    set_idt_gate(1, addr_isr22, 0x08, 0x8E);

    extern "C" fn isr23() {}
    let isr23: unsafe extern "C" fn() = isr23;
    let addr_isr23 = isr23 as usize as u64;
    set_idt_gate(1, addr_isr23, 0x08, 0x8E);

    extern "C" fn isr24() {}
    let isr24: unsafe extern "C" fn() = isr24;
    let addr_isr24 = isr24 as usize as u64;
    set_idt_gate(1, addr_isr24, 0x08, 0x8E);

    extern "C" fn isr25() {}
    let isr25: unsafe extern "C" fn() = isr25;
    let addr_isr25 = isr25 as usize as u64;
    set_idt_gate(1, addr_isr25, 0x08, 0x8E);

    extern "C" fn isr26() {}
    let isr26: unsafe extern "C" fn() = isr26;
    let addr_isr26 = isr26 as usize as u64;
    set_idt_gate(1, addr_isr26, 0x08, 0x8E);

    extern "C" fn isr27() {}
    let isr27: unsafe extern "C" fn() = isr27;
    let addr_isr27 = isr27 as usize as u64;
    set_idt_gate(1, addr_isr27, 0x08, 0x8E);

    extern "C" fn isr28() {}
    let isr28: unsafe extern "C" fn() = isr28;
    let addr_isr28 = isr28 as usize as u64;
    set_idt_gate(1, addr_isr28, 0x08, 0x8E);

    extern "C" fn isr29() {}
    let isr29: unsafe extern "C" fn() = isr29;
    let addr_isr29 = isr29 as usize as u64;
    set_idt_gate(1, addr_isr29, 0x08, 0x8E);

    extern "C" fn isr30() {}
    let isr30: unsafe extern "C" fn() = isr30;
    let addr_isr30 = isr30 as usize as u64;
    set_idt_gate(1, addr_isr30, 0x08, 0x8E);

    extern "C" fn isr31() {}
    let isr31: unsafe extern "C" fn() = isr31;
    let addr_isr31 = isr31 as usize as u64;
    set_idt_gate(1, addr_isr31, 0x08, 0x8E);

    /*
        setIdtGate(32, (uint32_t)irq0, 0x08, 0x8E);
        setIdtGate(33, (uint32_t)irq1, 0x08, 0x8E);
        setIdtGate(34, (uint32_t)irq2, 0x08, 0x8E);
        setIdtGate(35, (uint32_t)irq3, 0x08, 0x8E);
        setIdtGate(36, (uint32_t)irq4, 0x08, 0x8E);
        setIdtGate(37, (uint32_t)irq5, 0x08, 0x8E);
        setIdtGate(38, (uint32_t)irq6, 0x08, 0x8E);
        setIdtGate(39, (uint32_t)irq7, 0x08, 0x8E);
        setIdtGate(40, (uint32_t)irq8, 0x08, 0x8E);
        setIdtGate(41, (uint32_t)irq9, 0x08, 0x8E);
        setIdtGate(42, (uint32_t)irq10, 0x08, 0x8E);
        setIdtGate(43, (uint32_t)irq11, 0x08, 0x8E);
        setIdtGate(44, (uint32_t)irq12, 0x08, 0x8E);
        setIdtGate(45, (uint32_t)irq13, 0x08, 0x8E);
        setIdtGate(46, (uint32_t)irq14, 0x08, 0x8E);
        setIdtGate(47, (uint32_t)irq15, 0x08, 0x8E);


        setIdtGate(128, (uint32_t)isr128, 0x08, 0x8E); //System calls
        setIdtGate(177, (uint32_t)isr177, 0x08, 0x8E); //System calls
    */
    unsafe {
        asm!(
            "lidt [{}]", in(reg) &IDT_PTR, options(readonly, nostack, preserves_flags)
        );
        //asm!("sti")
    }
}

/*

struct idt_entry_struct idt_entries[256];
struct idt_ptr_struct idt_ptr;

extern void idt_flush(uint32_t);

void initIdt(){
    //0x20 commands and 0x21 data
    //0xA0 commands and 0xA1 data
    out_port_b(0x20, 0x11);
    out_port_b(0xA0, 0x11);

    out_port_b(0x21, 0x20);
    out_port_b(0xA1, 0x28);

    out_port_b(0x21,0x04);
    out_port_b(0xA1,0x02);

    out_port_b(0x21, 0x01);
    out_port_b(0xA1, 0x01);

    out_port_b(0x21, 0x0);
    out_port_b(0xA1, 0x0);

    setIdtGate(0, (uint32_t)isr0,0x08, 0x8E);
    setIdtGate(1, (uint32_t)isr1,0x08, 0x8E);
    setIdtGate(2, (uint32_t)isr2,0x08, 0x8E);
    setIdtGate(3, (uint32_t)isr3,0x08, 0x8E);
    setIdtGate(4, (uint32_t)isr4, 0x08, 0x8E);
    setIdtGate(5, (uint32_t)isr5, 0x08, 0x8E);
    setIdtGate(6, (uint32_t)isr6, 0x08, 0x8E);
    setIdtGate(7, (uint32_t)isr7, 0x08, 0x8E);
    setIdtGate(8, (uint32_t)isr8, 0x08, 0x8E);
    setIdtGate(9, (uint32_t)isr9, 0x08, 0x8E);
    setIdtGate(10, (uint32_t)isr10, 0x08, 0x8E);
    setIdtGate(11, (uint32_t)isr11, 0x08, 0x8E);
    setIdtGate(12, (uint32_t)isr12, 0x08, 0x8E);
    setIdtGate(13, (uint32_t)isr13, 0x08, 0x8E);
    setIdtGate(14, (uint32_t)isr14, 0x08, 0x8E);
    setIdtGate(15, (uint32_t)isr15, 0x08, 0x8E);
    setIdtGate(16, (uint32_t)isr16, 0x08, 0x8E);
    setIdtGate(17, (uint32_t)isr17, 0x08, 0x8E);
    setIdtGate(18, (uint32_t)isr18, 0x08, 0x8E);
    setIdtGate(19, (uint32_t)isr19, 0x08, 0x8E);
    setIdtGate(20, (uint32_t)isr20, 0x08, 0x8E);
    setIdtGate(21, (uint32_t)isr21, 0x08, 0x8E);
    setIdtGate(22, (uint32_t)isr22, 0x08, 0x8E);
    setIdtGate(23, (uint32_t)isr23, 0x08, 0x8E);
    setIdtGate(24, (uint32_t)isr24, 0x08, 0x8E);
    setIdtGate(25, (uint32_t)isr25, 0x08, 0x8E);
    setIdtGate(26, (uint32_t)isr26, 0x08, 0x8E);
    setIdtGate(27, (uint32_t)isr27, 0x08, 0x8E);
    setIdtGate(28, (uint32_t)isr28, 0x08, 0x8E);
    setIdtGate(29, (uint32_t)isr29, 0x08, 0x8E);
    setIdtGate(30, (uint32_t)isr30, 0x08, 0x8E);
    setIdtGate(31, (uint32_t)isr31, 0x08, 0x8E);

    setIdtGate(32, (uint32_t)irq0, 0x08, 0x8E);
    setIdtGate(33, (uint32_t)irq1, 0x08, 0x8E);
    setIdtGate(34, (uint32_t)irq2, 0x08, 0x8E);
    setIdtGate(35, (uint32_t)irq3, 0x08, 0x8E);
    setIdtGate(36, (uint32_t)irq4, 0x08, 0x8E);
    setIdtGate(37, (uint32_t)irq5, 0x08, 0x8E);
    setIdtGate(38, (uint32_t)irq6, 0x08, 0x8E);
    setIdtGate(39, (uint32_t)irq7, 0x08, 0x8E);
    setIdtGate(40, (uint32_t)irq8, 0x08, 0x8E);
    setIdtGate(41, (uint32_t)irq9, 0x08, 0x8E);
    setIdtGate(42, (uint32_t)irq10, 0x08, 0x8E);
    setIdtGate(43, (uint32_t)irq11, 0x08, 0x8E);
    setIdtGate(44, (uint32_t)irq12, 0x08, 0x8E);
    setIdtGate(45, (uint32_t)irq13, 0x08, 0x8E);
    setIdtGate(46, (uint32_t)irq14, 0x08, 0x8E);
    setIdtGate(47, (uint32_t)irq15, 0x08, 0x8E);


    setIdtGate(128, (uint32_t)isr128, 0x08, 0x8E); //System calls
    setIdtGate(177, (uint32_t)isr177, 0x08, 0x8E); //System calls

    idt_flush((uint32_t)&idt_ptr);

}



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

void irq_handler(struct InterruptRegisters* regs){
    void (*handler)(struct InterruptRegisters *regs);

    handler = irq_routines[regs->int_no - 32];

    if (handler){
        handler(regs);
    }

    if (regs->int_no >= 40){
        out_port_b(0xA0, 0x20);
    }

    out_port_b(0x20,0x20);
}*/
