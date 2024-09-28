# Makefile
QEMU = qemu-system-riscv64

G++ = riscv64-unknown-elf-g++
G++_ARGS = -nostdlib
G++_ARGS += -nostartfiles --static

BUILD_DIR = build


# QEMU_ARGS += -cpu shakti-c
# Must specify a machine type - from the qemu documentation
QEMU_ARGS += -machine virt
# QEMU_ARGS += -vga std
QEMU_ARGS += -bios build/thing.elf
QEMU_ARGS += -serial stdio
QEMU_ARGS += -device virtio-vga

#Source Files

#####################
# Hello World Example
#####################

HELLO_WORLD_DIR = hello_world

#Souce Files
HELLO_WORLD_SRC_CPP = $(wildcard $(HELLO_WORLD_DIR)/*.cpp)
HELLO_WORLD_SRC_S = $(wildcard $(HELLO_WORLD_DIR)/*.s)

#Object Files
HELLO_WORLD_OBJS = $(patsubst $(HELLO_WORLD_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(HELLO_WORLD_SRC_CPP))
HELLO_WORLD_OBJS += $(patsubst $(HELLO_WORLD_DIR)/%.s,$(BUILD_DIR)/%.o,$(HELLO_WORLD_SRC_S))

# Executable Name
HELLO_WORLD_TARGET = $(BUILD_DIR)/hello_world.elf

#Linker File
HELLO_WORLD_LINKER = $(HELLO_WORLD_DIR)/linker.ld

#####################
# SBI
#####################

SBI_DIR = sbi

#Souce Files
SBI_SRC_CPP = $(wildcard $(SBI_DIR)/*.cpp)
SBI_SRC_S = $(wildcard $(SBI_DIR)/*.s)

#Object Files
SBI_OBJS = $(patsubst $(SBI_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(SBI_SRC_CPP))
SBI_OBJS += $(patsubst $(SBI_DIR)/%.s,$(BUILD_DIR)/%.o,$(SBI_SRC_S))

# Executable Name
SBI_TARGET = $(BUILD_DIR)/sbi.elf

#Linker File
SBI_LINKER = $(SBI_DIR)/linker.ld



.PHONY: hello sbi run clean

hello: clean $(HELLO_WORLD_TARGET)
	
# links all .o files.
$(HELLO_WORLD_TARGET): $(HELLO_WORLD_OBJS)
	$(G++) $(G++_ARGS) -T $(HELLO_WORLD_LINKER) -o $@ $^

#compiiles each .cpp
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.c
	@$(G++) -c $< -o $@

#assembles .s files
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.s
	@$(G++) -c $< -o $@

sbi: clean $(SBI_TARGET)
	
# links all .o files.
$(SBI_TARGET): $(SBI_OBJS)
	$(G++) $(G++_ARGS) -T $(SBI_LINKER) -o $@ $^

#compiles .cpp files
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.cpp
	@$(G++) -c $< -o $@

#assembles .s files
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.s
	@$(G++) -c $< -o $@

run:
	$(QEMU) $(QEMU_ARGS) -bios $(wildcard $(BUILD_DIR)/*.elf)

clean:
	rm -rf $(BUILD_DIR)/*