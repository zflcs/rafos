
TARGET := riscv64gc-unknown-none-elf
MODE := release
OBJDUMP := riscv64-unknown-elf-objdump

BUILD_PATH := ../target/$(TARGET)/$(MODE)
ELF := $(BUILD_PATH)/rafos-runtime
ASM := $(BUILD_PATH)/rafos-runtime.asm


clean:
	cargo clean

build:
	cargo build --release
	$(OBJDUMP) -S -t $(ELF) > $(ASM)


disasm: build
	$(OBJDUMP) -S -t $(ELF) > $(ASM)