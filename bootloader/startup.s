.section .text.boot, "ax"
.global _start
_start:
    csrr  a0, mhartid
    bnez  a0, core_loop

    la sp, stack_start
    la gp, global_pointer

    call main
    
core_loop:
    j core_loop
