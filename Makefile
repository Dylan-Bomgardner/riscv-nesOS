# Makefile
QEMU = qemu-system-riscv64

G++ = riscv64-unknown-elf-g++
G++_ARGS = -nostdlib
G++_ARGS += -nostartfiles
G++_ARGE += -ffreestanding
G++_ARGS += -mcmodel=medany
G++_ARGS += -march=rv64gc -mabi=lp64d
LINKER_SCRIPT=-Tsrc/lds/linker.lds
TYPE=debug
RUST_TARGET=./target/riscv64gc-unknown-none-elf/$(TYPE)
LIBS=-L$(RUST_TARGET)
SOURCES_ASM=$(wildcard src/asm/*.S)
LIB=-lrust -lgcc
OUT=thing.elf


BUILD_DIR = build


QEMU_ARGS += -cpu rv64 -smp 4 -m 128M
# Must specify a machine type - from the qemu documentation

# QEMU_ARGS += -nographic
QEMU_ARGS += -machine virt
# QEMU_ARGS += -vga std
QEMU_ARGS += -bios build/thing.elf
QEMU_ARGS += -serial stdio
QEMU_ARGS += -device virtio-gpu-device
QEMU_ARGS += -device virtio-net-device
# QEMU_ARGS += 

.PHONY: run clean rust
all: rust run

rust: 
	cargo +nightly build --target riscv64gc-unknown-none-elf 
	$(G++) $(G++_ARGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB) -o $(BUILD_DIR)/$(OUT)
run:
	$(QEMU) $(QEMU_ARGS) -bios $(BUILD_DIR)/$(OUT)
debug:
	$(QEMU) $(QEMU_ARGS) -bios $(BUILD_DIR)/$(OUT) -S -gdb tcp::1234

clean:
	rm -rf $(BUILD_DIR)/*