ENTRY(_start)
PHDRS { code PT_LOAD FLAGS(5); data PT_LOAD FLAGS(6); }
SECTIONS {
  . = 0x400000;
  .text : { KEEP(*(.text.entry)) *(.text .text.*) *(.rodata .rodata.*) } :code
  ASSERT(. <= 0x410000, "user code exceeds quota")
  . = 0x500000;
  .data : { BYTE(0); *(.data .data.*) *(.sdata .sdata.*) *(.got .got.*) } :data
  .bss : { *(.bss .bss.*) *(.sbss .sbss.*) *(COMMON) } :data
  ASSERT(. <= 0x508000, "user data overlaps stack")
  /DISCARD/ : { *(.eh_frame*) *(.comment) }
}
