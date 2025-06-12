#pragma once

#include <stdint.h>

void idt_init();
void idt_set_handler_keyboard(void (*handler)());
