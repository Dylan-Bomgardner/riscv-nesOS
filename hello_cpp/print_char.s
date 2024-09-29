.section .text
.global print_char

print_char:
    #load an h into a0
    # addi a0, x0, 0x68
    li a1, 0x10000000
    sb a0, 0(a1)
    ret
