#include <stddef.h>
#include <stdint.h>
#include "x86_64/gdt.h"
#include "x86_64/idt.h"
#include "x86_64/pic.h"

#define IDT_IRQ0_TIMER 0x20
#define IDT_IRQ1_KEYBOARD 0x21

#define IDT_GATE_PRESENT (1 << 7)
#define IDT_GATE_DPL0 (0b00 << 5)
#define IDT_GATE_DPL1 (0b01 << 5)
#define IDT_GATE_DPL2 (0b10 << 5)
#define IDT_GATE_DPL3 (0b11 << 5)
#define IDT_GATE_TYPE_INTERRUPT 0xE

#define IDT_ENTRY_TYPE_INTERRUPT (IDT_GATE_PRESENT | IDT_GATE_DPL0 | IDT_GATE_TYPE_INTERRUPT)

struct IdtEntry {
	uint16_t offset_low;
	uint16_t selector;
	uint8_t  ist;
	uint8_t  type;
	uint16_t offset_mid;
	uint32_t offset_high;
	uint32_t reserved;
} __attribute__((packed));

struct IdtPtr {
	uint16_t limit;
	uint64_t base;
} __attribute__((packed));

struct IdtEntry idt[256] __attribute__((aligned(16)));
struct IdtPtr idt_ptr;

void (*idt_handler_keyboard_user)();

extern void idt_load(struct IdtPtr* idt_ptr);

void idt_set_entry(uint8_t vector, uint64_t isr_addr, uint16_t selector, uint8_t type) {
	idt[vector] = (struct IdtEntry) {
		.offset_low = (uint16_t) (isr_addr >> 0),
		.selector = selector,
		.ist = 0,
		.type = type,
		.offset_mid = (uint16_t) (isr_addr >> 16),
		.offset_high = (uint32_t) (isr_addr >> 32),
		.reserved = 0,
	};
}

extern void idt_handler_keyboard_wrapped();

void idt_handler_keyboard() {
	if (idt_handler_keyboard_user != NULL) {
		idt_handler_keyboard_user();
	}
	
	pic_eoi_master();
}

void idt_init() {
	pic_remap();
	
	idt_ptr.limit = (sizeof(struct IdtEntry) * 256) - 1;
	idt_ptr.base = (uint64_t) &idt;
	
	idt_set_entry(IDT_IRQ1_KEYBOARD, (uint64_t) idt_handler_keyboard_wrapped, GDT_SELECTOR_CS_KERNEL, IDT_ENTRY_TYPE_INTERRUPT);
	
	idt_load(&idt_ptr);
	
	asm volatile("sti");
}

void idt_set_handler_keyboard(void (*handler)()) {
	idt_handler_keyboard_user = handler;
}
