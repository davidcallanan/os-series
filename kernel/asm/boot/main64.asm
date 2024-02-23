global long_mode_start
extern kernel_main

section .boottext exec
bits 64
long_mode_start:
    ; load null into all data segment registers
    mov ax, 0
    mov ss, ax
    mov ds, ax
    mov es, ax
    mov fs, ax
    mov gs, ax

    mov rax, QWORD 0xffff800000000000
    add rsp, rax

    mov rax, QWORD kernel_main
	call rax
    hlt
