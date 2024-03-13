.section .text
.code64

reloadSegments:
   // Reload CS register:
   PUSH 0x08                 // Push code segment to stack, 0x08 is a stand-in for your code segment
   LEA RAX, [rip + reloadCS]       // Load address of .reload_CS into RAX
   PUSH RAX                  // Push this value to the stack
   RETFQ                     // Perform a far return, RETFQ or LRETQ depending on syntax
reloadCS:
   // Reload data segment registers
   MOV   AX, 0x10 // 0x10 is a stand-in for your data segment
   MOV   DS, AX
   MOV   ES, AX
   MOV   FS, AX
   MOV   GS, AX
   MOV   SS, AX
   RET