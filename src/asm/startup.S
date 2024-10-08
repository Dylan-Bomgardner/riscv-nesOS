.section .data
.section .text.boot, "ax"

.global _start
_start:
    csrr  a0, mhartid
    bnez  a0, core_loop

    la sp, _stack_start
    la gp, _global_pointer
    mv a2, a1

#Initialize BSS, inspo from Stephen Marz
    la 		a0, _bss_start
	la		a1, _bss_end
	bgeu	a0, a1, 2f
1:
	sd		zero, (a0)
	addi	a0, a0, 8
	bltu	a0, a1, 1b
2:

    la		t2, asm_trap_vector
	csrw	mtvec, t2
    call k_init
    
core_loop:
    j core_loop
