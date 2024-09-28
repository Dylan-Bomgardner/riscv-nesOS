
QEMU = qemu-system-riscv64

G++ = riscv64-unknown-elf-g++
G++_ARGS = -nostdlib
G++_ARGS += -nostartfiles --static

HELLO_WORLD_DIR = hello_world
BUILD_DIR = build


# QEMU_ARGS += -cpu shakti-c
# # Must specify a machine type - from the qemu documentation
QEMU_ARGS += -machine virt
# QEMU_ARGS += -vga std
QEMU_ARGS += -bios build/thing.elf
QEMU_ARGS += -serial stdio
QEMU_ARGS += -device virtio-vga

#Source Files
# HELLO_WORLD_SRC = $(wildcard $(HELLO_WORLD_DIR/*.c))
HELLO_WORLD_SRC_CPP = $(wildcard $(HELLO_WORLD_DIR)/*.cpp)
HELLO_WORLD_SRC_S = $(wildcard $(HELLO_WORLD_DIR)/*.s)
# HELLO_WORLD_S_SRC = $(wildcard $(HELLO_WORLD_DIR)/*.s)
#Object Files
HELLO_WORLD_OBJS = $(patsubst $(HELLO_WORLD_DIR)/%.cpp,$(BUILD_DIR)/%.o,$(HELLO_WORLD_SRC_CPP))
HELLO_WORLD_OBJS += $(patsubst $(HELLO_WORLD_DIR)/%.s,$(BUILD_DIR)/%.o,$(HELLO_WORLD_SRC_S))

# Executable Names
HELLO_WORLD_TARGET = $(BUILD_DIR)/hello_world.elf

#Linker Files
HELLO_WORLD_LINKER = $(HELLO_WORLD_DIR)/linker.ld

.PHONY: hello run clean

hello: $(HELLO_WORLD_TARGET)
	
# links all .o files.
# creates the file in the name $(HELLO_WORLD_TARGET)
$(HELLO_WORLD_TARGET): $(HELLO_WORLD_OBJS)
	$(G++) $(G++_ARGS) -T $(HELLO_WORLD_LINKER) -o $@ $^

# $(G++) -o $@ $^

#compiiles each .c file into a .o file without linking.
$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.c
# echo $(HELLO_WORLD_SRC)
	@$(G++) -c $< -o $@

$(BUILD_DIR)/%.o: $(HELLO_WORLD_DIR)/%.s
	@$(G++) -c hello_world/hello_world.s -o build/hello_world.o

run:
	$(QEMU) $(QEMU_ARGS)

clean:
	rm -rf $(BUILD_DIR)/*