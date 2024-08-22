/* the errata involves an instruction appearing at an address with its lower 12 bits set to 0xFFC */
/* starting the memory region at this offset lets us place the instruction at the required address */
MEMORY {
    RAM : ORIGIN = 0xFF0, LENGTH = 8K
}

ENTRY(_start)
SECTIONS {
    .text : { *(.text .text*) } > RAM
    . = ALIGN(8);
    .bss : { *(.bss .bss*) } > RAM
    . = ALIGN(8);
}
