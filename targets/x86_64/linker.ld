ENTRY(start)

KERNEL_START = 1M;
KERNEL_OFFSET = 0xffff800000000000;

SECTIONS
{
	. = KERNEL_START;

	.boot :
	{
		KEEP(*(.multiboot_header))
	}

	.boottext : 
	{
		*(.boottext)
	}

	.bootbss :
	{
		*(.bootbss .bootbss.*)
	}

	.bootrodata :
	{
		*(.bootrodata .bootrodata.*)
	}

	. += KERNEL_OFFSET;

	.kerneltext ALIGN(4K): AT(ADDR(.kerneltext) - KERNEL_OFFSET)
	{
		*:*(.text .text.*)
	}

	.kernelrodata ALIGN(4K): AT(ADDR(.kernelrodata) - KERNEL_OFFSET)
	{
		*:*(.rodata .rodata.*)
	}
	
	.kerneldata ALIGN(4K): AT(ADDR(.kerneldata) - KERNEL_OFFSET)
	{
		*:*(.data .data.*)
	}

	.kernelbss ALIGN(4K): AT(ADDR(.kernelbss) - KERNEL_OFFSET)
	{
		*:*(.bss .bss.*)
	}
}