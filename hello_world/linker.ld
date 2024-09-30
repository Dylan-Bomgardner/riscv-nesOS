ENTRY(_start)
MEMORY {
  dram_space (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS {
  /* .text and .rodata just go straight into the ROM. We don't need
     to mutate them ever. */
    .text : { 
        KEEP(.text.boot)
        KEEP(*(.text))
    } > dram_space
    .rodata : { *(.rodata) } >dram_space

  /* .bss doesn't have any "loadable" content, so it goes straight
     into RAM. We could include `AT> rom`, but because the sections
     have no content, it doesn't matter. */
  .bss : { *(.bss) } > dram_space

  /* As described above, we need to get a RAM VMA but a ROM LMA;
     the > and AT> operators achieve this. */
  .data : { *(.data) } > dram_space
}

/* The initialization code will need some symbols to know how to
   zero the .bss and copy the initial .data values. We can use the
   functions from the previous section for this! */

bss_start = ADDR(.bss);
bss_end = bss_start + SIZEOF(.bss);

data_start = ADDR(.data);
data_end = data_start + SIZEOF(.data);

rodata_start = ADDR(.rodata);
rodata_end = rodata_start + SIZEOF(.rodata);