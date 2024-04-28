// https://wiki.osdev.org/Getting_to_Ring_3#sysret_method
// https://www.amd.com/content/dam/amd/en/documents/processor-tech-docs/programmer-references/24593.pdf p. 174
// https://wiki.osdev.org/SYSENTER

.section .text
.code64

.globl jump_usermode
jump_usermode:
	push rdx // push rdx (3rd call parameter) as it will be overwritten until its used

	// enable system call extensions that enable sysret and syscall
	mov rcx, 0xc0000080
	rdmsr
	or eax, 1
	wrmsr

	// define SYSRET SYSCALL CS and SS and 32-bit SYSCALL Target EIP (latter is not needed I think)
	mov rcx, 0xc0000081
	rdmsr
	mov rax, 0x00000000
	mov rdx, 0x00130008
	// super weird
	// mov rdx, 0b0000000000010011...    SYSRET_CS = value + 16 and SYSRET_SS = value + 8 -> this means also in GDT the data segment has to come before code segment!
	//			...__0000000000001000 // SYSCALL_CS = value and SYSCALL_SS = value + 8
	wrmsr

	// define a handler for syscalls and write it to lstar register// attention: wrmsr writes edx:eax and not rax!
	mov rcx, 0xc0000082
	lea rax, [rip + syscall_handler]
	mov edx, 0xffff8000
	wrmsr

	pop rdx
	mov rcx, rdx // to be loaded into RIP
	mov r11, 0x202 // to be loaded into EFLAGS

	// Attention: CR3 consumes physical addresses!
	mov cr3, rdi
	
	mov rsp, rsi

	
	// TODO Investigate: Only enable interrupts here (pretty late), because otherwise during jump to usermode interrupt may appear, leading to weird behavior
	sti

	sysretq //use "o64 sysret" if you assemble with NASM

syscall_handler:
	cli // TODO Hack: No task switches during syscalls (due to stack issues)
	
    swapgs
	
	// FIXME SECURITY somewhere here we should change the stack pointer to kernel space; see e.g. https://supercip971.github.io/02-wingos-syscalls.html

	push rcx // syscall has set ecx to the rip of the userland process
	push r11 // syscall has set r11 to the rflags

	lea rax, [rip + system_call]
	call rax

	pop r11
	pop rcx 

    swapgs

	sti

	sysretq
