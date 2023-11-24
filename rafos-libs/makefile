TARGET := riscv64gc-unknown-linux-gnu
MODE := release
PWD := .
TARGET_DIR := ../target/$(TARGET)/$(MODE)
LIBS := $(wildcard $(PWD)/rafos-*)
ELFS := $(patsubst $(PWD)/rafos-%, $(TARGET_DIR)/librafos_%.so, $(LIBS))

OBJDUMP := riscv64-unknown-linux-gnu-objdump

FEATURE := no_std

build: 
	@cargo build --$(MODE) --features $(FEATURE)

disasm: build
	@$(foreach elf, $(ELFS), $(OBJDUMP) -S -t $(elf) > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)



.PHONY: test