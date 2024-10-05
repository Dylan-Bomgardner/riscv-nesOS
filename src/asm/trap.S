# trap.S
# In the future our trap vector will go here.

.global asm_trap_vector
# This will be our trap vector when we start
# handling interrupts.
asm_trap_vector:
	#print an h
	li a0, 0x68
	#write to 0x10000000
	li a1, 0x10000000
	sd a0, 0(a1)
	mret
