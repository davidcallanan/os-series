; https://github.com/scprogramming/Jazz2.0/blob/main/src/interrupts/idt.s
; https://wiki.osdev.org/ISR
; https://wiki.osdev.org/Interrupts_Tutorial

extern isr_handler
extern irq_handler

%macro ISR_NOERRCODE 1
    global isr%1
    isr%1:
        cli
        push LONG 0
        push LONG %1
        jmp isr_common_stub
%endmacro

%macro ISR_ERRCODE 1
    global isr%1
    isr%1:
        cli
        push LONG %1
        jmp isr_common_stub
%endmacro

%macro IRQ 2
    global irq%1
    irq%1:
        cli
        push LONG 0
        push LONG %2
        jmp irq_common_stub
%endmacro

ISR_NOERRCODE 0
ISR_NOERRCODE 1
ISR_NOERRCODE 2
ISR_NOERRCODE 3
ISR_NOERRCODE 4
ISR_NOERRCODE 5
ISR_NOERRCODE 6
ISR_NOERRCODE 7

ISR_ERRCODE 8
ISR_NOERRCODE 9 
ISR_ERRCODE 10
ISR_ERRCODE 11
ISR_ERRCODE 12
ISR_ERRCODE 13
ISR_ERRCODE 14

ISR_NOERRCODE 15
ISR_NOERRCODE 16
ISR_ERRCODE 17
ISR_NOERRCODE 18
ISR_NOERRCODE 19
ISR_NOERRCODE 20
ISR_ERRCODE 21
ISR_NOERRCODE 22
ISR_NOERRCODE 23
ISR_NOERRCODE 24
ISR_NOERRCODE 25
ISR_NOERRCODE 26
ISR_NOERRCODE 27
ISR_NOERRCODE 28
ISR_ERRCODE 29
ISR_ERRCODE 30
ISR_NOERRCODE 31
ISR_NOERRCODE 128
ISR_NOERRCODE 177

IRQ   0,    32
IRQ   1,    33
IRQ   2,    34
IRQ   3,    35
IRQ   4,    36
IRQ   5,    37
IRQ   6,    38
IRQ   7,    39
IRQ   8,    40
IRQ   9,    41
IRQ  10,    42
IRQ  11,    43
IRQ  12,    44
IRQ  13,    45
IRQ  14,    46
IRQ  15,    47

; https://www.reddit.com/r/osdev/comments/cp40lb/64bit_isr_handler_breaking_my_stack
; https://github.com/rust-osdev/x86_64/issues/392#issuecomment-1257883895

isr_common_stub:
	; https://aaronbloomfield.github.io/pdr/book/x86-64bit-ccc-chapter.pdf
	push rdi ; save previous value to stack as we are gonna using it to pass arguments to isr_handler
	push rsi

	mov rdi, [rsp+3*8]	; put the the error number into rsi (1nd argument for isr_handler); it has been previously pushed onto the stack (see macros above)
	mov rsi, [rsp+2*8]	; put the the isr number into rdi (2nd argument for isr_handler); it has been previously pushed onto the stack (see macros above)

	call isr_handler

	pop rsi
	pop rdi

	add rsp, 16 ; "pop" the two longs we have pushed originally
	sti
	iretq

irq_common_stub:
	call irq_handler

	add rsp, 16 ; "pop" the two longs we have pushed originally
	sti
	iretq