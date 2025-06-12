#pragma once

#define GDT_RING_0 0
#define GDT_RING_1 1 // legacy
#define GDT_RING_2 2 // legacy
#define GDT_RING_3 3

#define GDT_SELECTOR_INDEX 3

#define GDT_SELECTOR_CS_KERNEL (1 << GDT_SELECTOR_INDEX) | GDT_RING_0
