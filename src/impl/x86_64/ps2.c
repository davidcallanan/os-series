#include "x86_64/port.h"

#define PORT_PS2_DATA 0x60

uint8_t ps2_read_scan_code() {
	return port_inb(PORT_PS2_DATA);
}
