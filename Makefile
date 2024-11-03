# Makefile
BUILD_DIR = build

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

DTB_FILE = $(BUILD_DIR)/qemu.dtb
DTC_FILE = $(BUILD_DIR)/qemu.dtc



QEMU_ARGS += -cpu rv64 -smp 4 -m 128M
# Must specify a machine type - from the qemu documentation

# QEMU_ARGS += -nographic
QEMU_ARGS += -machine virt
# QEMU_ARGS += -vga std
QEMU_ARGS += -bios build/thing.elf
QEMU_ARGS += -serial stdio
# QEMU_ARGS += -monitor stdio
QEMU_ARGS += -device virtio-vga
QEMU_ARGS += -device virtio-net-pci
# QEMU_ARGS +=

.PHONY: run clean compile dtc run_graphics

all: compile rungraphics

compile:
	cargo build --target riscv64gc-unknown-none-elf
	$(G++) $(G++_ARGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB) -o $(BUILD_DIR)/$(OUT)

run: compile
	$(QEMU) $(QEMU_ARGS) -nographic -monitor none -bios $(BUILD_DIR)/$(OUT)

rungraphics:
	$(QEMU) $(QEMU_ARGS) -bios $(BUILD_DIR)/$(OUT)

debug: compile
	@echo "Ctrl-A C for QEMU console, then quit to exit"
	$(QEMU) $(QEMU_ARGS) -bios $(BUILD_DIR)/$(OUT) -S -gdb tcp::1234

dtc:
	$(QEMU) $(QEMU_ARGS) -machine dumpdtb=$(DTB_FILE)
	 dtc -I dtb -O dts $(DTB_FILE) -o $(DTC_FILE)

clean:
	rm -rf $(BUILD_DIR)/*
