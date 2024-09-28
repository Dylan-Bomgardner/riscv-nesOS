ENTRY(_start)
MEMORY {
  dram_space (rwx) : ORIGIN = 0x80000000, LENGTH = 128
}

SECTIONS {
  .text : {
    *(.text.bios)
  } > dram_space
}