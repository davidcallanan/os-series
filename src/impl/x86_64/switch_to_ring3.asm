; https://wiki.osdev.org/Getting_to_Ring_3#sysret_method
; https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf p. 174
; https://wiki.osdev.org/SYSENTER

; note: this code is for 64-bit long mode only.
;       it is unknown if it works in protected mode.
;       using intel assembly style

global jump_usermode
extern userland
extern TSS_ENTRY
jump_usermode:
	; enable system call extensions that enable sysret and syscall
	mov rcx, 0xc0000080
	rdmsr
	or eax, 1
	wrmsr

	; define SYSRET SYSCALL CS and SS and 32-bit SYSCALL Target EIP (latter is not needed I think)
	mov rcx, 0xc0000081
	rdmsr
	mov rax, 0x00000000
	mov rdx, 0x00130008
	; super weird
	; mov rdx, 0b0000000000010011...    SYSRET_CS = value + 16 and SYSRET_SS = value + 8 -> this means also in GDT the data segment has to come before code segment!
	;			...__0000000000001000 ; SYSCALL_CS = value and SYSCALL_SS = value + 8
	wrmsr

	; define a handler for syscalls and write it to lstar register
	mov rcx, 0xc0000082
	mov rax, syscall_handler
	mov rdx, 0x0
	wrmsr

	mov ecx, userland ; to be loaded into RIP
	mov r11, 0x202 ; to be loaded into EFLAGS

	o64 sysret ;use "o64 sysret" if you assemble with NASM

extern system_call
extern userland_loop
syscall_handler:
    swapgs

    call system_call

    swapgs
    sti

	mov ecx, userland_loop ; to be loaded into RIP
	mov r11, 0x202 ; to be loaded into EFLAGS
	o64 sysret