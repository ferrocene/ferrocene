/* the nuttx ELF loader will use the first symbol in `.text` as the entry point so */
/* make sure `main` ends up first */
SECTIONS {
  .text 0 : {
    *(.text.main)
    *(.text)
    *(.text.*)
  }
}
