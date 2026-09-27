.section .text
.global _start
_start:
    cli
    lea rsp, [rip + BOOT_STACK + 32768]
    xor rbp, rbp
    call kernel_main
1:  hlt
    jmp 1b

.global load_segments
load_segments:
    push 8
    lea rax, [rip + 2f]
    push rax
    retfq
2:  mov ax, 16
    mov ds, ax
    mov es, ax
    mov ss, ax
    xor eax, eax
    mov fs, ax
    mov gs, ax
    mov ax, 40
    ltr ax
    ret

.macro ISR n
.global isr\n
isr\n:
.if (\n != 8) && (\n != 10) && (\n != 11) && (\n != 12) && (\n != 13) && (\n != 14) && (\n != 17) && (\n != 21) && (\n != 29) && (\n != 30)
    push 0
.endif
    push \n
    jmp trap_entry
.endm
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,128
    ISR \n
.endr

trap_entry:
    cld
    push rax
    push rbx
    push rcx
    push rdx
    push rbp
    push rdi
    push rsi
    push r8
    push r9
    push r10
    push r11
    push r12
    push r13
    push r14
    push r15
    mov rdi, rsp
    and rsp, -16
    call trap
    mov rdi, rax
.global restore
restore:
    mov rsp, rdi
    pop r15
    pop r14
    pop r13
    pop r12
    pop r11
    pop r10
    pop r9
    pop r8
    pop rsi
    pop rdi
    pop rbp
    pop rdx
    pop rcx
    pop rbx
    pop rax
    add rsp, 16
    iretq

.global idle
idle:
    sti
    hlt
    jmp idle

.section .rodata
.global isr_table
isr_table:
.irp n,0,1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31,32,128
    .quad isr\n
.endr

# All user programs are position independent and copied into separate RX pages.
.global worker_start, worker_end
worker_start:
    mov rbx, 0x500000
    mov byte ptr [rbx], 0x11
1:  cmp byte ptr [rbx], 0x11
    jne 2f
    pause
    jmp 1b
2:  mov rax, 5
    mov rdi, 0xe0
    int 0x80
    jmp 2b
worker_end:

.global peer_start, peer_end
peer_start:
    mov rbx, 0x500000
    cmp byte ptr [rbx], 0
    jne 3f
    mov byte ptr [rbx], 0x22
    mov rax, 5
    mov rdi, 0x21
    int 0x80
1:  mov rax, 5
    mov rdi, 0x23
    int 0x80
    mov rax, 3
    int 0x80
    cmp byte ptr [rbx], 0x22
    jne 3f
    pause
    jmp 1b
3:  mov rax, 5
    mov rdi, 0xe1
    int 0x80
    jmp 3b
peer_end:

.global attack_start, attack_end
attack_start:
    mov rax, 2
    mov rdi, -1
    int 0x80
    cmp rax, -1
    jne 2f
    mov rax, 5
    mov rdi, 0x22
    int 0x80
    mov rax, [r12]
2:  mov rax, 5
    mov rdi, 0xe2
    int 0x80
    jmp 2b
attack_end:

.global write_start, write_end
write_start:
    mov rax, 0x400000
    mov byte ptr [rax], 0x90
1:  mov rax, 5
    mov rdi, 0xe3
    int 0x80
    jmp 1b
write_end:

.global nx_start, nx_end
nx_start:
    mov rax, 0x500000
    mov byte ptr [rax], 0xc3
    call rax
1:  mov rax, 5
    mov rdi, 0xe4
    int 0x80
    jmp 1b
nx_end:
