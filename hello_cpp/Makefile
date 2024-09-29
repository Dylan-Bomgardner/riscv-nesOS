# Compiler and assembler
CC = riscv64-unknown-elf-g++
AS = riscv64-unknown-elf-as

# Source files
C_SRC = main.cpp
ASM_SRC = print_char.s

# Output executable
OUTPUT = a.out

# Flags
CFLAGS = -Wall -mabi=lp64 -march=rv64im -nostartfiles
LDFLAGS = -T riscv64-virt.ld

.PHONY: all clean run

all: $(OUTPUT)

$(OUTPUT): $(C_SRC) $(ASM_SRC)
	$(AS) -o print_char.o $(ASM_SRC)          # Assemble the assembly file
	$(CC) $(CFLAGS) -o $(OUTPUT) $(C_SRC) print_char.o $(LDFLAGS)  # Link C and assembly files

run: $(OUTPUT)
	qemu-system-riscv64 -machine virt -m 128M -gdb tcp::1234 -S -kernel $(OUTPUT) -

clean:
	rm -f $(OUTPUT) print_char.o
