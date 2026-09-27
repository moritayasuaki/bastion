ENTRY(_start)
SECTIONS {
 . = 0x80200000;
 .text : { KEEP(*(.text.entry)) *(.text .text.*) }
 .rodata : ALIGN(4096) { *(.rodata .rodata.*) }
 .data : ALIGN(4096) { *(.data .data.*) *(.sdata .sdata.*) *(.got .got.*) }
 .bss : ALIGN(4096) { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) }
 /DISCARD/ : { *(.eh_frame*) *(.comment) }
}
