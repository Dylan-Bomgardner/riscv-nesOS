/* Script for -z combreloc */
/* Copyright (C) 2014-2024 Free Software Foundation, Inc.
   Copying and distribution of this script, with or without modification,
   are permitted in any medium without royalty provided the copyright
   notice and this notice are preserved.  */
OUTPUT_FORMAT("elf64-littleriscv", "elf64-littleriscv",
	      "elf64-littleriscv")
OUTPUT_ARCH(riscv)
/* riscv64-virt.ld */
ENTRY(_start)

MEMORY
{
    RAM (rw) : ORIGIN = 0x80000000, LENGTH = 256K  /* 256KB RAM for the program */
    ROM (rx) : ORIGIN = 0x80040000, LENGTH = 512K  /* 512KB ROM for read-only data */
}

/* Sections for the linker */
SECTIONS
{
    . = ORIGIN(RAM); /* Start at the beginning of RAM */

    .text : {
        *(.text) /* All .text sections from all object files */
    }

    .rodata : {
        *(.rodata) /* All .rodata sections from all object files */
    }

    .data : {
        *(.data) /* All .data sections from all object files */
    }

    .bss : {
        *(.bss) /* All .bss sections from all object files */
        _end = .; /* End of BSS section */
    }

    /* Stack section */
    .stack (NOLOAD) : {
        . = ALIGN(8);
        __stack_top = . + 0x1000; /* Define stack top (1KB for stack) */
    }

    /* Global pointer */
    __global_pointers = .; /* Set the global pointer */
}
