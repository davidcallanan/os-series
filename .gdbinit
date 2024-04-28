set output-radix 16
set disassembly-flavor intel
add-symbol-file build/userspace/x86_64-unknown-none/debug/helloworld
display/5i $pc
display/20xg $sp

b isr_common_stub
#b irq_common_stub
#b *(irq_common_stub+44)
b *0x0
b *0x2
b *0x3
b *0x4
b *0x5

#set logging on
#set height 0

define log_instructions    
  while $pc >= 0x1000
    stepi
  end
end