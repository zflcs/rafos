TARGET := riscv64gc-unknown-linux-gnu
MODE := release
APP_DIR := src/bin
TARGET_DIR := ../target/$(TARGET)/$(MODE)
APPS := $(wildcard $(APP_DIR)/*.rs)
ELFS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%, $(APPS))
BINS := $(patsubst $(APP_DIR)/%.rs, $(TARGET_DIR)/%.bin, $(APPS))

OBJDUMP := riscv64-unknown-linux-gnu-objdump
OBJCOPY := riscv64-unknown-linux-gnu-objcopy


elf: $(APPS)
	@LOG=DEBUG cargo build --$(MODE)


binary: elf
	@$(foreach elf, $(ELFS), $(OBJCOPY) $(elf) --strip-all -O binary $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.bin, $(elf));)
	@$(foreach elf, $(ELFS), $(OBJDUMP) -S -t $(elf) > $(patsubst $(TARGET_DIR)/%, $(TARGET_DIR)/%.asm, $(elf));)


build: binary

clean:
	@cargo clean

.PHONY: elf binary build clean