TARGET := riscv64gc-unknown-linux-gnu
MODE := release
APP_DIR := src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))

QEMU := qemu-riscv64
OBJDUMP := riscv64-unknown-linux-gnu-objdump

runtime:
	@cd ../rafos-libs && make build

elf: $(APPS)
	@cargo build --$(MODE)

test: elf runtime
	@$(foreach elf, $(ELFS), $(OBJDUMP) -S -t $(elf) > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)
	$(foreach elf, $(ELFS), $(QEMU) -cpu rv64 $(elf))



.PHONY: test