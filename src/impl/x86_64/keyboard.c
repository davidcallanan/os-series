#include <stddef.h>
#include "bool.h"
#include "keyboard.h"
#include "x86_64/idt.h"
#include "x86_64/ps2.h"

#define KEYBOARD_EXTENDED_SCAN_CODE 0xE0

void (*keyboard_handler_user)(struct KeyboardEvent event);

void keyboard_handler() {
	static bool is_extended = 0;
	
	uint8_t scan_code = ps2_read_scan_code();
	
	if (scan_code == KEYBOARD_EXTENDED_SCAN_CODE) {
		is_extended = true;
		return;
	}
	
	if (keyboard_handler_user == NULL) {
		return;
	}
	
	uint16_t fat_code = scan_code & 0x7F;
	
	if (is_extended) {
		is_extended = false;
		fat_code |= KEYBOARD_EXTENDED_SCAN_CODE << 8;
	}
	
	struct KeyboardEvent event;
	
	if ((scan_code & 0x80) == 0) {
		event.type = KEYBOARD_EVENT_TYPE_MAKE;
	} else {
		event.type = KEYBOARD_EVENT_TYPE_BREAK;
	}
	
	event.code = fat_code;
	
	keyboard_handler_user(event);
}

void keyboard_init() {
	idt_init();
	idt_set_handler_keyboard(keyboard_handler);
}

void keyboard_set_handler(void (*handler)(struct KeyboardEvent event)) {
	keyboard_handler_user = handler;	
}
