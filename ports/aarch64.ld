ENTRY(_start)
SECTIONS {
 . = 0x40080000;
 .text : { KEEP(*(.text.entry)) *(.text .text.*) }
 .rodata : ALIGN(4096) { *(.rodata .rodata.*) }
 .data : ALIGN(4096) { *(.data .data.*) *(.got .got.*) }
 .bss : ALIGN(4096) { *(.bss .bss.*) *(COMMON) }
 /DISCARD/ : { *(.eh_frame*) *(.comment) }
}
