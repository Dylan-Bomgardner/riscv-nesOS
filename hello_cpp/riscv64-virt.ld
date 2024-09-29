MEMORY
{
  RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}
ENTRY(_start)
SECTIONS
{
  .text ALIGN(4K) :
  {
    *(.text .text.* .gnu.linkonce.t.*)
    KEEP (*(.init))
    KEEP (*(.fini))
    PROVIDE(__stack_top = . + 0xFFF & ~0xFFF);
  } > RAM

  .rodata ALIGN(4K) : { *(.rodata .rodata.* .gnu.linkonce.r.*) } > RAM

  .data ALIGN(4K) :
  {
    __DATA_BEGIN__ = .;
    *(.data .data.* .gnu.linkonce.d.*)
    SORT(CONSTRUCTORS)
  } > RAM

  .sdata ALIGN(4K) :
  {
    __global_pointer$ = . + 0x800;  # Set global pointer to the start of .sdata section + offset
    *(.sdata .sdata.* .gnu.linkonce.s.*)
  } > RAM

  .bss ALIGN(4K) :
  {
    __bss_start = .;
    *(.bss .bss.* .gnu.linkonce.b.*)
    *(COMMON)
    __bss_end = .;
  } > RAM

  PROVIDE (__etext = .);
  PROVIDE (_etext = .);
  PROVIDE (etext = .);
  PROVIDE (_edata = .);
  PROVIDE (edata = .);
  PROVIDE (_end = .);
  PROVIDE (end = .);
}