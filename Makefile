# Makefile
QEMU = qemu-system-riscv64

G++ = riscv64-unknown-elf-g++
G++_ARGS = -nostdlib
G++_ARGS += -nostartfiles
G++_ARGE += -ffreestanding
G++_ARGS += -mcmodel=medany
G++_ARGS += -march=rv64gc -mabi=lp64d
LINKER_SCRIPT=-Tsrc/lds/virt.lds
TYPE=debug
RUST_TARGET=./build/riscv64gc-unknown-none-elf/$(TYPE)
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

#Source Files


#####################
# SBI
#####################

SBI_DIR = bootloader

#Souce Files
SBI_SRC_CPP = $(wildcard $(SBI_DIR)/*.cpp)
SBI_SRC_S = $(wildcard $(SBI_DIR)/*.s)

#Object Files
SBI_OBJS = $(patsubst $(SBI_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(SBI_SRC_CPP))
SBI_OBJS += $(patsubst $(SBI_DIR)/%.s,$(BUILD_DIR)/%.o,$(SBI_SRC_S))

# Executable Name
SBI_TARGET = $(BUILD_DIR)/bootloader.elf

#Linker File
SBI_LINKER = $(SBI_DIR)/linker.ld



.PHONY: hello sbi run clean rust

rust: 
	cargo +nightly build --target riscv64gc-unknown-none-elf --target-dir build
	$(G++) $(G++_ARGS) $(LINKER_SCRIPT) $(INCLUDES) -o $(OUT) $(SOURCES_ASM) $(LIBS) $(LIB) -o $(BUILD_DIR)/$(OUT)
sbi: clean $(SBI_TARGET)
	
# links all .o files.
$(SBI_TARGET): $(SBI_OBJS)
	$(G++) $(G++_ARGS) -T $(SBI_LINKER) -o $@ $^

#compiles .cpp files
$(BUILD_DIR)/%.o: $(SBI_DIR)/%.cpp
	@$(G++) -c $< -o $@

#assembles .s files
$(BUILD_DIR)/%.o: $(SBI_DIR)/%.s
	@$(G++) -c $< -o $@


hello: clean $(HELLO_WORLD_TARGET)
	
# links all .o files.
$(HELLO_WORLD_TARGET): $(HELLO_WORLD_OBJS)
	$(G++) $(G++_ARGS) -T $(HELLO_WORLD_LINKER) -o $@ $^

#compiiles each .cpp
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.c
	$(G++) -c $< -o $@

#assembles .s files
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.s
	$(G++) -c $< -o $@


run:
	$(QEMU) $(QEMU_ARGS) -bios $(BUILD_DIR)/$(OUT)

clean:
	rm -rf $(BUILD_DIR)/*