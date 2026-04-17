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
    /* this section is produced on AArch64 Linux ELFs and cannot be discarded */
    /* because it's referred to from other section. it instead needs to be */
    /* placed in the output ELF to avoid linker errors so we place it RAM */
    /* even though it won't be inspected as part of the test */
    .interp : { *(.interp .interp*) } > RAM
}
