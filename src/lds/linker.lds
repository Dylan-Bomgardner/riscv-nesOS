ENTRY(_start)
MEMORY {
  ram  (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

heap_size = 1M;

SECTIONS {
  /* .text and .rodata just go straight into the ROM. We don't need
     to mutate them ever. */
   . = ORIGIN(ram);

   .text.boot : {
      KEEP(*(.text.boot))
   }

   .text : { 
     KEEP(*(.text))
   }
   
   . = ALIGN(8);
   /* .bss doesn't have any "loadable" content, so it goes straight
      into RAM. We could include `AT> rom`, but because the sections
      have no content, it doesn't matter. */
   .bss : { *(.bss) }
   . = ALIGN(8);

   . = ALIGN(8);
   .rodata : { *(.rodata) }
   /* As described above, we need to get a RAM VMA but a ROM LMA;
      the > and AT> operators achieve this. */
   . = ALIGN(8);
   
   .data : { 
      _global_pointer = .;
      *(.data)   
   }
   . = ALIGN(8);

   /* Heap */
   _heap_start = .;
}

/* The initialization code will need some symbols to know how to
   zero the .bss and copy the initial .data values. We can use the
   functions from the previous section for this! */

_bss_start = ADDR(.bss);
_bss_end = _bss_start + SIZEOF(.bss);

data_start = ADDR(.data);
data_end = data_start + SIZEOF(.data);

rom_data_start = LOADADDR(.data);

_stack_start = ORIGIN(ram) + LENGTH(ram);

_heap_end = _heap_start + heap_size;