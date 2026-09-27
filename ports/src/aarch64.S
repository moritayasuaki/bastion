.section .text.entry
.global _start
_start:
    msr daifset, #15
    ldr x0, =KSTACK+65536
    mov sp, x0
    bl kernel_main
1:  wfi
    b 1b

.section .text
.balign 2048
.global vectors
vectors:
.irp number,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15
    sub sp, sp, #288
    stp x0,x1,[sp,#0]
    mov x0,#\number
    b exception
    .balign 128
.endr
exception:
    str x0,[sp,#272]
    stp x2,x3,[sp,#16]
    stp x4,x5,[sp,#32]
    stp x6,x7,[sp,#48]
    stp x8,x9,[sp,#64]
    stp x10,x11,[sp,#80]
    stp x12,x13,[sp,#96]
    stp x14,x15,[sp,#112]
    stp x16,x17,[sp,#128]
    stp x18,x19,[sp,#144]
    stp x20,x21,[sp,#160]
    stp x22,x23,[sp,#176]
    stp x24,x25,[sp,#192]
    stp x26,x27,[sp,#208]
    stp x28,x29,[sp,#224]
    str x30,[sp,#240]
    mrs x1,sp_el0
    mrs x2,elr_el1
    mrs x3,spsr_el1
    stp x1,x2,[sp,#248]
    str x3,[sp,#264]
    mov x0,sp
    bl trap
.global restore
restore:
    mov x30,x0
    ldp x1,x2,[x30,#248]
    ldr x3,[x30,#264]
    msr sp_el0,x1
    msr elr_el1,x2
    msr spsr_el1,x3
    ldr x0,=KSTACK+65536
    mov sp,x0
    ldp x0,x1,[x30,#0]
    ldp x2,x3,[x30,#16]
    ldp x4,x5,[x30,#32]
    ldp x6,x7,[x30,#48]
    ldp x8,x9,[x30,#64]
    ldp x10,x11,[x30,#80]
    ldp x12,x13,[x30,#96]
    ldp x14,x15,[x30,#112]
    ldp x16,x17,[x30,#128]
    ldp x18,x19,[x30,#144]
    ldp x20,x21,[x30,#160]
    ldp x22,x23,[x30,#176]
    ldp x24,x25,[x30,#192]
    ldp x26,x27,[x30,#208]
    ldp x28,x29,[x30,#224]
    ldr x30,[x30,#240]
    eret
.global idle
idle:
    wfi
    b idle
